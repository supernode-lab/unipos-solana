use anchor_lang::prelude::*;
use anchor_lang::{account, emit, require, AnchorDeserialize, AnchorSerialize, Key};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::{Core, UniposError};

pub fn deposit_security(ctx: Context<DepositSecurity>, amount: u64) -> Result<()> {
    let core = &mut ctx.accounts.core;

    // Transfer tokens from provider to core
    let cpi_accounts = Transfer {
        from: ctx.accounts.provider_token_account.to_account_info(),
        to: ctx.accounts.core_vault.to_account_info(),
        authority: ctx.accounts.provider.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, amount)?;

    core.total_security_deposit += amount;
    core.allowed_collateral = get_collateral_by_security_deposit(
        core.total_security_deposit,
        core.apy,
        core.lock_period,
    );

    emit!(SecurityDepositedEvent {
            amount,
            total_security: core.total_security_deposit,
        });

    Ok(())
}

pub fn withdraw_security(ctx: Context<WithdrawSecurity>, amount: u64) -> Result<()> {
    let core = &mut ctx.accounts.core;

    let remaining_collateral = core.allowed_collateral - core.total_collateral;
    let withdrawable_security = get_security_deposit_by_collateral(
        remaining_collateral,
        core.apy,
        core.lock_period,
    );
    require!(withdrawable_security >= amount, UniposError::InsufficientSecurity);

    core.total_security_deposit -= amount;
    core.allowed_collateral = get_collateral_by_security_deposit(
        core.total_security_deposit,
        core.apy,
        core.lock_period,
    );
    let total_security_deposit = core.total_security_deposit;

    // Transfer tokens back to provider
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.core_vault.to_account_info(),
            to: ctx.accounts.provider_token_account.to_account_info(),
            authority: ctx.accounts.core.to_account_info(),
        }
    );
    let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
    token::transfer(transfer_ctx.with_signer(&[pda_sign]), amount)?;

    emit!(SecurityWithdrawnEvent {
            amount,
            remaining_security: total_security_deposit,
        });

    Ok(())
}

pub fn collect_from_pool(ctx: Context<CollectFromPool>) -> Result<()> {
    let core = &mut ctx.accounts.core;
    let vault = &mut ctx.accounts.core_vault;
    require!(core.total_collateral + core.total_security_deposit < vault.amount + core.unstaked_collateral + core.total_claimed_rewards, UniposError::NoLockedToken);
    let extra_token = vault.amount - (core.total_collateral + core.total_security_deposit - core.unstaked_collateral - core.total_claimed_rewards);

    // Transfer tokens back to provider
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.core_vault.to_account_info(),
            to: ctx.accounts.provider_token_account.to_account_info(),
            authority: ctx.accounts.core.to_account_info(),
        }
    );
    let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
    token::transfer(transfer_ctx.with_signer(&[pda_sign]), extra_token)?;

    emit!(CollectEvent{amount: extra_token});

    Ok(())
}

#[derive(Accounts)]
pub struct DepositSecurity<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump,
        has_one = provider,
    )]
    pub core: Account<'info, Core>,

    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub provider_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub provider: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawSecurity<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump,
        has_one = provider,
    )]
    pub core: Account<'info, Core>,
    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub provider_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub provider: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CollectFromPool<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump,
        has_one = admin,
        has_one = provider,
    )]
    pub core: Account<'info, Core>,
    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    /// CHECK: no need
    pub provider: AccountInfo<'info>,

    #[account(
        mut,
        token::authority = provider
    )]
    pub provider_token_account: Account<'info, TokenAccount>,

    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct SecurityDepositedEvent {
    pub amount: u64,
    pub total_security: u64,
}

#[event]
pub struct SecurityWithdrawnEvent {
    pub amount: u64,
    pub remaining_security: u64,
}

#[event]
pub struct CollectEvent {
    pub amount: u64,
}

fn get_collateral_by_security_deposit(security_deposit: u64, apy: u64, lock_days: u64) -> u64 {
    (security_deposit * 1_000_000_000) / ((apy * lock_days as u64) / 365)
}

fn get_security_deposit_by_collateral(collateral: u64, apy: u64, lock_days: u64) -> u64 {
    (collateral * ((apy * lock_days as u64) / 365)) / 1_000_000_000
}