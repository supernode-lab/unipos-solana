use crate::events::StakeInfoUpdated;
use crate::states::{Config, StakeInfo};
use anchor_lang::prelude::*;

pub fn set_stake_info_handler(
    ctx: Context<SetStakeInfo>,
    stakecores: &[Pubkey],
    ratios: &[u128],
    kpi_stakecore: Pubkey,
    kpi_ratio: u128,
) -> Result<()> {
    let config = &ctx.accounts.config;
    config.only_admin(ctx.accounts.signer.key())?;

    let stake_info = &mut ctx.accounts.stake_info;
    stake_info.set(stakecores, ratios, kpi_stakecore, kpi_ratio)?;

    emit!(StakeInfoUpdated {
        stakecores: stakecores.to_vec(),
        ratios: ratios.to_vec(),
        kpi_stakecore: kpi_stakecore,
        kpi_ratio: kpi_ratio,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SetStakeInfo<'info> {
    #[account(
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [StakeInfo::SEED],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    pub signer: Signer<'info>,
}
