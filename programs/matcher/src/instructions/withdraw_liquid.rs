use crate::errors::SwitchError;
use crate::events::LiquidWithdrawn;
use crate::states::{AssetInfo, Config, TOKEN_VAULT_SEED, VAULT_AUTHORITY_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn withdraw_liquid_handler(ctx: Context<WithdrawLiquid>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    config.only_provider(ctx.accounts.to_token_authority.key())?;

    let asset_info = &mut ctx.accounts.asset_info;
    let now = Clock::get()?.unix_timestamp as u64;
    require!(
        asset_info.last_deposit_time + config.liquid_lock_period < now,
        SwitchError::LiquidLocking
    );

    let available = asset_info.available_token();
    require!(available >= amount, SwitchError::InsufficientBalance);

    let seeds: &[&[&[u8]]] = &[&[VAULT_AUTHORITY_SEED, &[ctx.bumps.vault_authority]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    asset_info.total_liquid -= amount;

    emit!(LiquidWithdrawn {
        account: ctx.accounts.to_token_authority.key(),
        token_account: ctx.accounts.to_token_account.key(),
        amount: amount,
        remaining_liquid: asset_info.total_liquid,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquid<'info> {
    #[account(
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [AssetInfo::SEED],
        bump,
    )]
    pub asset_info: Box<Account<'info, AssetInfo>>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED],
        bump
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is a PDA authority, checked via seeds
    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = to_token_account.owner == *to_token_authority.key,
        constraint = to_token_account.mint == token_vault.mint,
    )]
    pub to_token_account: Box<Account<'info, TokenAccount>>,

    pub to_token_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
