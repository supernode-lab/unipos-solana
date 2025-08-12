use anchor_lang::prelude::*;
use anchor_lang::{account, emit, require, AnchorDeserialize, AnchorSerialize, Key};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::{Core, UniposError};

const SECONDS_PER_DAY: u128 = 86400;

pub fn deposit_security(ctx: Context<DepositSecurity>, amount: u64) -> Result<()> {
    let core = &mut ctx.accounts.core;

    let (collateral, security) = get_collateral_by_security_deposit(
        amount as u128,
        core.apy_percentage as u128,
        core.lock_period_secs as u128,
    )?;

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
    token::transfer(cpi_ctx, security)?;

    core.total_security_deposit = core.total_security_deposit.checked_add(security)
        .ok_or(UniposError::InvalidAmount)?;
    core.allowed_collateral = core.allowed_collateral.checked_add(collateral)
        .ok_or(UniposError::InvalidAmount)?;

    emit!(SecurityDepositedEvent {
            amount: security,
            total_security: core.total_security_deposit,
        });

    Ok(())
}

pub fn withdraw_security(ctx: Context<WithdrawSecurity>, amount: u64) -> Result<()> {
    let core = &mut ctx.accounts.core;

    let remaining_collateral = core.allowed_collateral.checked_sub(core.total_collateral)
        .ok_or(UniposError::InvalidAmount)?;
    let withdrawable_security = get_security_deposit_by_collateral(
        remaining_collateral as u128,
        core.apy_percentage as u128,
        core.lock_period_secs as u128,
    )?;
    require!(withdrawable_security >= amount, UniposError::InsufficientSecurity);

    let (collateral, security) = get_collateral_by_security_deposit(
        (core.total_security_deposit.checked_sub(amount)
            .ok_or(UniposError::InvalidAmount)?) as u128,
        core.apy_percentage as u128,
        core.lock_period_secs as u128,
    )?;
    core.allowed_collateral = collateral;
    let withdraw_amount = core.total_security_deposit.checked_sub(security)
        .ok_or(UniposError::InvalidAmount)?;
    core.total_security_deposit = security;

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
    token::transfer(transfer_ctx.with_signer(&[pda_sign]), withdraw_amount)?;

    emit!(SecurityWithdrawnEvent {
            amount: withdraw_amount,
            remaining_security: total_security_deposit,
        });

    Ok(())
}

pub fn collect_from_pool(ctx: Context<CollectFromPool>) -> Result<()> {
    let core = &mut ctx.accounts.core;
    let vault = &mut ctx.accounts.core_vault;
    
    let total_locked = core.total_collateral.checked_add(core.total_security_deposit)
        .ok_or(UniposError::InvalidAmount)?;
    let total_available = vault.amount.checked_add(core.unstaked_collateral)
        .ok_or(UniposError::InvalidAmount)?
        .checked_add(core.total_claimed_rewards)
        .ok_or(UniposError::InvalidAmount)?;
    
    require!(total_locked < total_available, UniposError::NoLockedToken);
    
    let total_deducted = total_locked.checked_sub(core.unstaked_collateral)
        .ok_or(UniposError::InvalidAmount)?
        .checked_sub(core.total_claimed_rewards)
        .ok_or(UniposError::InvalidAmount)?;
    
    let extra_token = vault.amount.checked_sub(total_deducted)
        .ok_or(UniposError::InvalidAmount)?;

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

// collateral * apy * lock_days / 360 = security
// lock_days = lock_secs / SECONDS_PER_DAY
// so collateral = security * 360 * SECONDS_PER_DAY * 100 / (apy_percentage * lock_secs)
fn get_collateral_by_security_deposit(security_deposit: u128, apy_percentage: u128, lock_secs: u128) -> Result<(u64, u64)> {
    let numerator = security_deposit.checked_mul(100)
        .ok_or(UniposError::InvalidAmount)?
        .checked_mul(360)
        .ok_or(UniposError::InvalidAmount)?
        .checked_mul(SECONDS_PER_DAY)
        .ok_or(UniposError::InvalidAmount)?;
    let denominator = apy_percentage.checked_mul(lock_secs)
        .ok_or(UniposError::InvalidAmount)?;
    let collateral = numerator.checked_div(denominator)
        .ok_or(UniposError::InvalidAmount)?;
    let mut security = security_deposit;
    if numerator.checked_rem(denominator).unwrap_or(0) > 0 {
        let adjusted_numerator = collateral.checked_mul(denominator)
            .ok_or(UniposError::InvalidAmount)?;
        let adjusted_denominator = 100u128.checked_mul(360)
            .ok_or(UniposError::InvalidAmount)?
            .checked_mul(SECONDS_PER_DAY)
            .ok_or(UniposError::InvalidAmount)?;
        security = adjusted_numerator.checked_div(adjusted_denominator)
            .ok_or(UniposError::InvalidAmount)?;
    }
    Ok((collateral as u64, security as u64))
}

// collateral * apy * lock_days / 360 = security
// lock_days = lock_secs / SECONDS_PER_DAY
fn get_security_deposit_by_collateral(collateral: u128, apy_percentage: u128, lock_secs: u128) -> Result<u64> {
    let numerator = collateral.checked_mul(apy_percentage)
        .ok_or(UniposError::InvalidAmount)?
        .checked_mul(lock_secs)
        .ok_or(UniposError::InvalidAmount)?;
    let denominator = 360u128.checked_mul(SECONDS_PER_DAY)
        .ok_or(UniposError::InvalidAmount)?
        .checked_mul(100)
        .ok_or(UniposError::InvalidAmount)?;
    Ok((numerator.checked_div(denominator)
        .ok_or(UniposError::InvalidAmount)?) as u64)
}

#[cfg(test)]
mod tests {
    use crate::security::{get_collateral_by_security_deposit, get_security_deposit_by_collateral};

    #[test]
    fn test_get_collateral_by_security_deposit() {
        let (collateral, security) = get_collateral_by_security_deposit(5000_000_000_000, 160, 15552000).unwrap();
        assert_eq!(security, 4999_999_999_999);
        assert_eq!(collateral, 6336_805_555_555);
    }

    #[test]
    fn test_get_security_deposit_by_collateral() {
        let security = get_security_deposit_by_collateral(6336_805_555_555, 160, 15552000).unwrap();
        assert_eq!(security, 4999_999_999_999);
    }
}