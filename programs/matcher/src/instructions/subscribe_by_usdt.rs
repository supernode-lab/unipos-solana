use crate::errors::SwitchError;
use crate::events::SubscribedByUSDT;
use crate::states::{
    AssetInfo, Config, PriceInfo, StakeInfo, TOKEN_VAULT_SEED, USDT_VAULT_SEED,
    VAULT_AUTHORITY_SEED,
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use common::*;
use unipos::calculate_user_rewards;
use unipos::cpi::stake;
use unipos::program::Unipos;

pub fn subscribe_by_usdt_handler<'a>(
    ctx: Context<'_, '_, '_, 'a, SubscribeByUsdt<'a>>,
    number: u64,
    cost: u64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    config.is_usdt_subscription_enabled()?;

    let price_info = &ctx.accounts.price_info;
    let token_amount = price_info.calc_token(cost);
    let need_usdt_amount = price_info.cal_usdt(token_amount);

    let asset_info = &mut ctx.accounts.asset_info;
    require!(
        token_amount <= asset_info.available_token(),
        SwitchError::InsufficientFund
    );

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.usdt_from.to_account_info(),
            to: ctx.accounts.usdt_vault.to_account_info(),
            authority: ctx.accounts.usdt_from_authority.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, need_usdt_amount)?;

    asset_info.exchanged += token_amount;
    asset_info.total_usdt += need_usdt_amount;

    _stake(ctx, number, token_amount, need_usdt_amount)?;
    Ok(())
}

fn _stake<'a>(
    ctx: Context<'_, '_, '_, 'a, SubscribeByUsdt<'a>>,
    mut number: u64,
    amount: u64,
    usdt_amount: u64,
) -> Result<()> {
    let stake_info = &*ctx.accounts.stake_info;
    let stake_num = stake_info.stake_num();
    if stake_num == 0 {
        Err(SwitchError::StakeInfoMissing)?;
    }

    let (kpi_num, kpi_rewards) = if stake_info.kpi_ratio == 0 {
        (0, 0)
    } else {
        (
            1,
            (amount as u128 * stake_info.kpi_ratio / PRECISION) as u64,
        )
    };

    let need_accounts_per_stake = 5;
    if ctx.remaining_accounts.len() != (stake_num + kpi_num) * need_accounts_per_stake {
        Err(SwitchError::InvalidParameter)?
    }

    let mut acc_principal: u64 = 0;
    let mut acc_rewards: u64 = kpi_rewards;

    let mut principals = Vec::with_capacity(stake_num);
    let mut rewards = Vec::with_capacity(stake_num);

    let core_accounts = ctx.remaining_accounts;
    for i in 0..stake_num {
        if stake_info.stakes[i] != *core_accounts[i * need_accounts_per_stake].key {
            Err(SwitchError::InvalidParameter)?;
        }

        let principal = if (i + 1) == stake_num {
            amount - acc_principal
        } else {
            (amount as u128 * stake_info.ratios[i] / PRECISION) as u64
        };

        let core_config = unipos::Core::try_from_slice(
            *ctx.remaining_accounts[i * need_accounts_per_stake + 1]
                .data
                .borrow(),
        )?;
        let reward = calculate_user_rewards(
            principal as u128,
            core_config.apy_percentage as u128,
            core_config.lock_period_secs as u128,
            core_config.user_reward_share as u128,
        )
        .or(Err(SwitchError::InvalidParameter))?;
        principals.push(principal);
        rewards.push(reward);
        acc_principal += principal;
        acc_rewards += reward;
    }

    if kpi_num == 1 {
        let config = &ctx.accounts.config;
        require!(
            stake_info.kpi_stake == *core_accounts[stake_num * need_accounts_per_stake].key,
            SwitchError::InvalidParameter
        );

        require!(
            config.beneficiary == *core_accounts[stake_num * need_accounts_per_stake + 4].key,
            SwitchError::InvalidParameter
        )
    }

    let asset_info = &ctx.accounts.asset_info;
    require!(
        acc_rewards <= asset_info.available_token(),
        SwitchError::InsufficientFund
    );

    let mut owners: Vec<Pubkey> = Vec::with_capacity(stake_num);
    let mut numbers: Vec<u64> = Vec::with_capacity(stake_num);

    let seeds: &[&[&[u8]]] = &[&[VAULT_AUTHORITY_SEED, &[ctx.bumps.vault_authority]]];
    for i in 0..stake_num {
        let cpi_program = &core_accounts[i * need_accounts_per_stake];
        let core_config = &core_accounts[i * need_accounts_per_stake + 1];
        let core_vault = &core_accounts[i * need_accounts_per_stake + 2];
        let core_staker_record = &core_accounts[i * need_accounts_per_stake + 3];
        let core_staker = &core_accounts[i * need_accounts_per_stake + 4];

        owners.push(core_staker.key());

        let cpi_deposit_security_account = unipos::cpi::accounts::DepositSecurity {
            core: core_config.to_account_info(),
            core_vault: core_vault.to_account_info(),
            provider_token_account: ctx.accounts.token_vault.to_account_info(),
            provider: ctx.accounts.vault_authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(cpi_program.clone(), cpi_deposit_security_account, seeds);
        unipos::cpi::deposit_security(cpi_ctx, rewards[i])?;

        let cpi_stake_accounts = unipos::cpi::accounts::Stake {
            core: core_config.to_account_info(),
            core_vault: core_vault.to_account_info(),
            staker_record: core_staker_record.to_account_info(),
            staker: core_staker.to_account_info(),
            user: ctx.accounts.vault_authority.to_account_info(),
            user_token_account: ctx.accounts.token_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_stake_accounts, seeds);
        unipos::cpi::stake(cpi_ctx, number, principals[i])?;

        numbers.push(number);

        number += 1;
    }

    if kpi_num == 1 {
        let cpi_program = &core_accounts[stake_num * need_accounts_per_stake];
        let core_config = &core_accounts[stake_num * need_accounts_per_stake + 1];
        let core_vault = &core_accounts[stake_num * need_accounts_per_stake + 2];
        let core_staker_record = &core_accounts[stake_num * need_accounts_per_stake + 3];
        let core_staker = &core_accounts[stake_num * need_accounts_per_stake + 4];

        let cpi_stake_accounts = unipos::cpi::accounts::Stake {
            core: core_config.to_account_info(),
            core_vault: core_vault.to_account_info(),
            staker_record: core_staker_record.to_account_info(),
            staker: core_staker.to_account_info(),
            user: ctx.accounts.vault_authority.to_account_info(),
            user_token_account: ctx.accounts.token_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_stake_accounts, seeds);
        unipos::cpi::stake(cpi_ctx, number, kpi_rewards)?;
    }

    let mut event = SubscribedByUSDT {
        account: ctx.accounts.usdt_from_authority.key(),
        usdt_amount: usdt_amount,
        price: ctx.accounts.price_info.price,
        owners: owners,
        numbers: numbers,
        token_amount: amount,
        kpi_number: 0,
        kpi_amount: 0,
    };

    if kpi_num == 1 {
        event.kpi_amount = kpi_rewards;
        event.kpi_number = number;
    }

    emit!(event);
    Ok(())
}

#[derive(Accounts)]
pub struct SubscribeByUsdt<'info> {
    #[account(
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        seeds = [StakeInfo::SEED],
        bump,
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

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
        constraint = usdt_from.owner == *usdt_from_authority.key,
        constraint = usdt_from.mint == usdt_vault.mint,
    )]
    pub usdt_from: Box<Account<'info, TokenAccount>>,

    pub usdt_from_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
