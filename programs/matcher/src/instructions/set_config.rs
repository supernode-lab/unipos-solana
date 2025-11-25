use crate::events::{
    AdminUpdated, BeneficiaryUpdated, LiquidLockPeriodUpdated, MinSubscribeAmountUpdated,
    ProviderAdded, ProviderRemoved, SubscriptionsUpdated,
};
use crate::states::Config;
use anchor_lang::prelude::*;
pub fn set_admin_handler(ctx: Context<SetConfig>, new_admin: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    let event = AdminUpdated {
        old_admin: config.admin,
        new_admin: new_admin,
    };

    config.admin = new_admin;

    emit!(event);

    Ok(())
}

pub fn add_provider_handler(ctx: Context<SetConfig>, new_provider: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let signer = ctx.accounts.signer.key;
    config.only_provider(*signer)?;
    config.add_provider(new_provider)?;

    emit!(ProviderAdded {
        account: *ctx.accounts.signer.key,
        new_provider: new_provider,
    });

    Ok(())
}

pub fn remove_provider_handler(ctx: Context<SetConfig>, provider: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let signer = ctx.accounts.signer.key;
    config.only_provider(*signer)?;
    config.remove_provider(provider)?;

    emit!(ProviderRemoved {
        account: *ctx.accounts.signer.key,
        provider: provider,
    });

    Ok(())
}

pub fn set_beneficiary_handler(ctx: Context<SetConfig>, new_beneficiary: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    config.beneficiary = new_beneficiary;

    emit!(BeneficiaryUpdated {
        new_beneficiary: new_beneficiary,
    });

    Ok(())
}

pub fn set_liquid_lock_period_handler(
    ctx: Context<SetConfig>,
    new_liquid_lock_period: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    let event = LiquidLockPeriodUpdated {
        old_liquid_lock_period: config.liquid_lock_period,
        new_liquid_lock_period: new_liquid_lock_period,
    };
    config.liquid_lock_period = new_liquid_lock_period;

    emit!(event);
    Ok(())
}

pub fn set_min_subscribe_amount_handler(ctx: Context<SetConfig>, amount: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    let event = MinSubscribeAmountUpdated {
        old_min_subscribe_amount: config.min_subscribe_amount,
        new_min_subscribe_amount: amount,
    };
    config.min_subscribe_amount = amount;

    emit!(event);

    Ok(())
}

pub fn set_subscriptions_handler(
    ctx: Context<SetConfig>,
    usdt_enabled: bool,
    token_enabled: bool,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    config.usdt_subscription_enabled = usdt_enabled;
    config.token_subscription_enabled = token_enabled;

    emit!(SubscriptionsUpdated {
        usdt_enabled: usdt_enabled,
        token_enabled: token_enabled,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct SetConfig<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    pub signer: Signer<'info>,
}
