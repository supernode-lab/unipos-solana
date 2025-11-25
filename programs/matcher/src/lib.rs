mod errors;
mod events;
pub mod instructions;
mod states;

use anchor_lang::prelude::*;

declare_id!("GBWC9S9Gw8B8NmEabfPcz4JgS9vBd9ETAHXpuK3H9MHh");

use instructions::*;

#[program]
pub mod matcher {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        liquid_lock_period: u64,
        min_subscribe_amount: u64,
        token_price: u128,
        usdt_subscription_enabled: bool,
        token_subscription_enabled: bool,
    ) -> Result<()> {
        initialize_handler(
            ctx,
            liquid_lock_period,
            min_subscribe_amount,
            token_price,
            usdt_subscription_enabled,
            token_subscription_enabled,
        )
    }

    pub fn withdraw_usdt(ctx: Context<WithdrawUsdt>, amount: u64) -> Result<()> {
        withdraw_usdt_handler(ctx, amount)
    }

    pub fn deposit_liquid(ctx: Context<DepositLiquid>, amount: u64) -> Result<()> {
        deposit_liquid_handler(ctx, amount)
    }

    pub fn withdraw_liquid(ctx: Context<WithdrawLiquid>, amount: u64) -> Result<()> {
        withdraw_liquid_handler(ctx, amount)
    }

    pub fn subscribe_by_usdt<'a>(
        ctx: Context<'_, '_, '_, 'a, SubscribeByUsdt<'a>>,
        number: u64,
        cost: u64,
    ) -> Result<()> {
        subscribe_by_usdt_handler(ctx, number, cost)
    }

    pub fn subscribe_by_token<'a>(
        ctx: Context<'_, '_, '_, 'a, SubscribeByToken<'a>>,
        number: u64,
        cost: u64,
    ) -> Result<()> {
        subscribe_by_token_handler(ctx, number, cost)
    }

    pub fn set_admin(ctx: Context<SetConfig>, new_admin: Pubkey) -> Result<()> {
        set_admin_handler(ctx, new_admin)
    }

    pub fn add_provider(ctx: Context<SetConfig>, new_provider: Pubkey) -> Result<()> {
        add_provider_handler(ctx, new_provider)
    }

    pub fn remove_provider(ctx: Context<SetConfig>, provider: Pubkey) -> Result<()> {
        remove_provider_handler(ctx, provider)
    }

    pub fn set_beneficiary(ctx: Context<SetConfig>, new_beneficiary: Pubkey) -> Result<()> {
        set_beneficiary_handler(ctx, new_beneficiary)
    }

    pub fn set_liquid_lock_period(
        ctx: Context<SetConfig>,
        new_liquid_lock_period: u64,
    ) -> Result<()> {
        set_liquid_lock_period_handler(ctx, new_liquid_lock_period)
    }

    pub fn set_min_subscribe_amount(ctx: Context<SetConfig>, amount: u64) -> Result<()> {
        set_min_subscribe_amount_handler(ctx, amount)
    }

    pub fn set_subscriptions(
        ctx: Context<SetConfig>,
        usdt_enabled: bool,
        token_enabled: bool,
    ) -> Result<()> {
        set_subscriptions_handler(ctx, usdt_enabled, token_enabled)
    }

    pub fn set_stake_info(
        ctx: Context<SetStakeInfo>,
        stakecores: Vec<Pubkey>,
        ratios: Vec<u128>,
        kpi_stakecore: Pubkey,
        kpi_ratio: u128,
    ) -> Result<()> {
        set_stake_info_handler(ctx, &stakecores, &ratios, kpi_stakecore, kpi_ratio)
    }

    pub fn set_price(ctx: Context<SetPrice>, new_price: u128) -> Result<()> {
        set_price_handler(ctx, new_price)
    }
}
