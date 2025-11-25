use crate::errors::SwitchError;
use crate::events::UsdtWithdrawn;
use crate::states::{AssetInfo, Config, PriceInfo, USDT_VAULT_SEED, VAULT_AUTHORITY_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
pub fn withdraw_usdt_handler(ctx: Context<WithdrawUsdt>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    config.only_provider(ctx.accounts.to_usdt_authority.key())?;

    let asset_info = &mut ctx.accounts.asset_info;

    let balance = asset_info.balance_usdt();
    require!(balance >= amount, SwitchError::InsufficientBalance);

    let seeds: &[&[&[u8]]] = &[&[VAULT_AUTHORITY_SEED, &[ctx.bumps.vault_authority]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.usdt_program.to_account_info(),
        Transfer {
            from: ctx.accounts.usdt_vault.to_account_info(),
            to: ctx.accounts.usdt_vault.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    asset_info.withdrawn_usdt += amount;

    emit!(UsdtWithdrawn {
        account: ctx.accounts.to_usdt_authority.key(),
        usdt_account: ctx.accounts.to_usdt_account.key(),
        amount: amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawUsdt<'info> {
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
        seeds = [PriceInfo::SEED],
        bump,
    )]
    pub price_info: Box<Account<'info, PriceInfo>>,

    #[account(
        mut,
        seeds = [USDT_VAULT_SEED],
        bump
    )]
    pub usdt_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is a PDA authority, checked via seeds
    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = to_usdt_account.owner == *to_usdt_authority.key,
        constraint = to_usdt_account.mint == usdt_vault.mint,
    )]
    pub to_usdt_account: Box<Account<'info, TokenAccount>>,

    pub to_usdt_authority: Signer<'info>,

    pub usdt_program: Program<'info, Token>,
}
