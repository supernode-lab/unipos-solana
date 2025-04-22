use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, AccountInfo, Context, Program, Signer, System};
use anchor_spl::associated_token::AssociatedToken;
use crate::{Core, UniposError};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::stakeholder::{StakeholderInfo, MAX_STAKEHOLDERS};

pub fn stake(ctx: Context<Stake>, number: u64, amount: u64) -> Result<()> {
    require!(amount > 0, UniposError::InvalidAmount);
    require!(amount >= ctx.accounts.core.min_stake_amount, UniposError::AmountTooSmall);
    require!(
        ctx.accounts.core.total_collateral + amount <= ctx.accounts.core.allowed_collateral,
        UniposError::InsufficientAllowance
    );

    // Transfer tokens from user to core
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.core_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, amount)?;

    // Create new stake record
    let staker_record = &mut ctx.accounts.staker_record;
    staker_record.staker = ctx.accounts.staker.key();
    staker_record.collateral = amount;
    staker_record.start_time = Clock::get()?.unix_timestamp as u64;
    staker_record.lock_period = ctx.accounts.core.lock_period;
    staker_record.locked_rewards = calculate_user_rewards(
        amount,
        ctx.accounts.core.apy,
        ctx.accounts.core.lock_period,
        ctx.accounts.core.user_reward_share,
    );
    staker_record.claimed_rewards = 0;
    staker_record.unstaked = 0;

    // Update stake core state
    let core = &mut ctx.accounts.core;
    core.total_collateral += amount;

    emit!(StakeEvent {
            user: ctx.accounts.user.key(),
            amount,
            start_time: Clock::get()?.unix_timestamp as u64,
            lock_days: ctx.accounts.core.lock_period,
        });

    Ok(())
}

pub fn unstake(ctx: Context<Unstake>, number: u64) -> Result<()> {
    let staker_record = &mut ctx.accounts.staker_record;
    require!(staker_record.staker == ctx.accounts.user.key(), UniposError::NotOwner);

    require!(
        Clock::get()?.unix_timestamp as u64 >= staker_record.start_time + staker_record.lock_period,
        UniposError::LockPeriodNotEnded
    );
    require!(staker_record.unstaked == 0, UniposError::AlreadyClaimed);

    staker_record.unstaked = 1;
    let core = &mut ctx.accounts.core;
    core.unstaked_collateral += staker_record.collateral;

    // Transfer tokens back to user
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.core_vault.to_account_info(),
            to: ctx.accounts.staker_vault.to_account_info(),
            authority: ctx.accounts.core.to_account_info(),
        }
    );
    let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
    token::transfer(transfer_ctx.with_signer(&[pda_sign]), staker_record.collateral)?;

    emit!(UnstakeEvent {
            user: ctx.accounts.user.key(),
            amount: staker_record.collateral,
        });

    Ok(())
}

pub fn claim_rewards(ctx: Context<ClaimRewards>, number: u64) -> Result<()> {
    let staker_record = &mut ctx.accounts.staker_record;
    let total_unlocked = get_unlocked_installment_rewards(
        staker_record,
        ctx.accounts.core.installment_num,
    );
    require!(staker_record.claimed_rewards < total_unlocked, UniposError::NothingToClaim);

    let to_be_claimed = total_unlocked - staker_record.claimed_rewards;
    staker_record.claimed_rewards += to_be_claimed;
    staker_record.locked_rewards -= to_be_claimed;

    let core = &mut ctx.accounts.core;
    core.total_claimed_rewards += to_be_claimed;

    // Calculate beneficiary share
    let beneficiary_share = (to_be_claimed * (100 - core.user_reward_share)) / core.user_reward_share;
    core.beneficiary_total_rewards += beneficiary_share;

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.core_vault.to_account_info(),
            to: ctx.accounts.staker_vault.to_account_info(),
            authority: ctx.accounts.core.to_account_info(),
        }
    );
    let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
    token::transfer(transfer_ctx.with_signer(&[pda_sign]), to_be_claimed)?;


    emit!(RewardsClaimedEvent {
            user: ctx.accounts.user.key(),
            amount: to_be_claimed,
        });

    Ok(())
}

#[derive(Accounts)]
#[instruction(number: u64, amount: u64)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump
    )]
    pub core: Box<Account<'info, Core>>,

    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 * 7 + 2 + 4 + 32 * 64,
        seeds = [b"staker_record", staker.key().as_ref(), number.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_record: Box<Account<'info, StakerRecord>>,

    /// CHECK: no need
    pub staker: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        token::mint = mint,
        token::authority = core,
        seeds = [b"staker_vault", staker.key().as_ref()],
        bump,
    )]
    pub staker_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(number: u64)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"core"],
        bump
    )]
    pub core: Account<'info, Core>,

    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    /// CHECK: no need
    pub staker: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"staker_record", staker.key().as_ref(), number.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_record: Account<'info, StakerRecord>,

    #[account(
        mut,
        seeds = [b"staker_vault", staker.key().as_ref()],
        bump,
    )]
    pub staker_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


#[derive(Accounts)]
#[instruction(number: u64)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump
    )]
    pub core: Account<'info, Core>,

    #[account(
        mut,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    /// CHECK: no need
    pub staker: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"staker_record", staker.key().as_ref(), number.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_record: Account<'info, StakerRecord>,

    #[account(
        mut,
        seeds = [b"staker_vault", staker.key().as_ref()],
        bump,
    )]
    pub staker_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}


#[account]
pub struct StakerRecord {
    pub staker: Pubkey,
    pub collateral: u64,
    pub start_time: u64,
    pub lock_period: u64,
    pub locked_rewards: u64,
    pub claimed_rewards: u64,
    pub unstaked: u8,

    pub granted_reward: u64,
    pub granted_collateral: u64,
    pub stakeholders: Vec<StakeholderInfo>,
    pub stakeholders_cnt: u8,
}

#[event]
pub struct StakeEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub start_time: u64,
    pub lock_days: u64,
}

#[event]
pub struct UnstakeEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RewardsClaimedEvent {
    pub user: Pubkey,
    pub amount: u64,
}

fn calculate_user_rewards(amount: u64, apy: u64, lock_period: u64, user_reward_share: u64) -> u64 {
    let total_rewards = (amount * apy * lock_period as u64) / (365 * 86400);
    (total_rewards * user_reward_share) / 100
}

impl Default for StakerRecord {
    fn default() -> Self {
        let mut r = StakerRecord::default();
        r.stakeholders = Vec::with_capacity(MAX_STAKEHOLDERS as usize);
        r.stakeholders.resize(MAX_STAKEHOLDERS as usize, Default::default());
        r
    }
}

fn get_unlocked_installment_rewards(staker_record: &StakerRecord, installment_num: u64) -> u64 {
    let total_rewards = staker_record.claimed_rewards + staker_record.locked_rewards;
    let elapsed_time = Clock::get().unwrap().unix_timestamp as u64 - staker_record.start_time;
    let unlocked_phase = if elapsed_time >= staker_record.lock_period {
        installment_num
    } else {
        (elapsed_time * installment_num) / staker_record.lock_period
    };
    (total_rewards / installment_num) * unlocked_phase
}
