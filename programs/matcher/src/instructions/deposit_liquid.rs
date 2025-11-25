use crate::events::LiquidDeposited;
use crate::states::{AssetInfo, Config, TOKEN_VAULT_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn deposit_liquid_handler(ctx: Context<DepositLiquid>, amount: u64) -> Result<()> {
    ctx.accounts
        .config
        .only_provider(ctx.accounts.from_token_authority.key())?;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.from_token_authority.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;

    let asset_info = &mut ctx.accounts.asset_info;
    asset_info.total_liquid += amount;
    asset_info.last_deposit_time = Clock::get()?.unix_timestamp as u64;

    emit!(LiquidDeposited {
        account: ctx.accounts.from_token_account.key(),
        amount: amount,
        total_liquid: asset_info.total_liquid,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct DepositLiquid<'info> {
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

    #[account(
        mut,
        constraint = from_token_account.owner == *from_token_authority.key,
        constraint = from_token_account.mint == token_vault.mint,
    )]
    pub from_token_account: Box<Account<'info, TokenAccount>>,

    pub from_token_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
