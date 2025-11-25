use crate::events::PriceUpdated;
use crate::states::{Config, PriceInfo};
use anchor_lang::prelude::*;

pub fn set_price_handler(ctx: Context<SetPrice>, new_price: u128) -> Result<()> {
    let config = &ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    let price_info = &mut ctx.accounts.price_info;

    let event = PriceUpdated {
        old_price: price_info.price,
        new_price: new_price,
    };

    price_info.price = new_price;

    emit!(event);

    Ok(())
}

#[derive(Accounts)]
pub struct SetPrice<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [PriceInfo::SEED],
        bump,
    )]
    pub price_info: Box<Account<'info, PriceInfo>>,

    pub signer: Signer<'info>,
}
