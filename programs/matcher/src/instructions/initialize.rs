use crate::states::VAULT_AUTHORITY_SEED;
use crate::states::{AssetInfo, Config, PriceInfo, StakeInfo, TOKEN_VAULT_SEED, USDT_VAULT_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn initialize_handler(
    ctx: Context<Initialize>,
    liquid_lock_period: u64,
    min_subscribe_amount: u64,
    token_price: u128,
    usdt_subscription_enabled: bool,
    token_subscription_enabled: bool,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.providers.push(ctx.accounts.provider.key());
    config.beneficiary = ctx.accounts.beneficiary.key();
    config.token_mint = ctx.accounts.token_mint.key();
    config.usdt_mint = ctx.accounts.usdt_mint.key();
    config.liquid_lock_period = liquid_lock_period;
    config.min_subscribe_amount = min_subscribe_amount;
    config.usdt_subscription_enabled = usdt_subscription_enabled;
    config.token_subscription_enabled = token_subscription_enabled;

    let price_info = &mut ctx.accounts.price_info;
    price_info.price = token_price;
    price_info.usdt_decimals = ctx.accounts.usdt_mint.decimals;
    price_info.token_decimals = ctx.accounts.token_mint.decimals;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = fee_payer,
        space = Config::LEN,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = fee_payer,
        space = StakeInfo::LEN,
        seeds = [StakeInfo::SEED],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
        init,
        payer = fee_payer,
        space = AssetInfo::LEN,
        seeds = [AssetInfo::SEED],
        bump,
    )]
    pub asset_info: Box<Account<'info, AssetInfo>>,

    #[account(
        init,
        payer = fee_payer,
        space = PriceInfo::LEN,
        seeds = [PriceInfo::SEED],
        bump,
    )]
    pub price_info: Box<Account<'info, PriceInfo>>,

    #[account(
        init,
        payer = fee_payer,
        token::mint = token_mint,
        token::authority = vault_authority,
        seeds = [TOKEN_VAULT_SEED],
        bump
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = fee_payer,
        token::mint = usdt_mint,
        token::authority = vault_authority,
        seeds = [USDT_VAULT_SEED],
        bump
    )]
    pub usdt_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is a PDA authority, checked via seeds
    #[account(
        seeds = [VAULT_AUTHORITY_SEED],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    /// CHECK:
    pub admin: UncheckedAccount<'info>,
    /// CHECK:
    pub provider: UncheckedAccount<'info>,
    /// CHECK:
    pub beneficiary: UncheckedAccount<'info>,

    pub token_mint: Box<Account<'info, Mint>>,
    pub usdt_mint: Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
