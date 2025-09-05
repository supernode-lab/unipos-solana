use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, AccountInfo, Context, Program, Signer, System};
use anchor_spl::associated_token::AssociatedToken;
use crate::{Core, UniposError};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::stakeholder::{StakeholderInfo, MAX_STAKEHOLDERS};

pub fn stake(ctx: Context<Stake>, number: u64, amount: u64) -> Result<()> {
    require!(amount > 0, UniposError::InvalidAmount);
    require!(amount >= ctx.accounts.core.min_stake_amount, UniposError::AmountTooSmall);
    let new_total = ctx.accounts.core.total_collateral.checked_add(amount).ok_or_else(|| UniposError::InsufficientAllowance)?;

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
    staker_record.start_time = Clock::get()?.unix_timestamp as u64;
    staker_record.lock_period_secs = ctx.accounts.core.lock_period_secs;
    staker_record.locked_rewards = amount;
    staker_record.claimed_rewards = 0;
    staker_record.unstaked = 0;
    staker_record.number = number;

    // Update stake core state
    let core = &mut ctx.accounts.core;
    core.total_collateral = new_total;

    emit!(StakeEvent {
            user: ctx.accounts.user.key(),
            amount,
            start_time: Clock::get()?.unix_timestamp as u64,
            lock_days: ctx.accounts.core.lock_period_secs,
        });

    Ok(())
}

pub fn claim_rewards(ctx: Context<ClaimRewards>, number: u64) -> Result<()> {
    let staker_record = &mut ctx.accounts.staker_record;
    let total_unlocked = get_unlocked_installment_rewards(
        Clock::get().unwrap().unix_timestamp as u64,
        staker_record,
        ctx.accounts.core.installment_num,
        ctx.accounts.core.cliff_period_secs,
    )?;
    require!(staker_record.claimed_rewards < total_unlocked, UniposError::NothingToClaim);

    let to_be_claimed = total_unlocked.checked_sub(staker_record.claimed_rewards)
        .ok_or(UniposError::InvalidAmount)?;
    staker_record.claimed_rewards = staker_record.claimed_rewards.checked_add(to_be_claimed)
        .ok_or(UniposError::InvalidAmount)?;
    staker_record.locked_rewards = staker_record.locked_rewards.checked_sub(to_be_claimed)
        .ok_or(UniposError::InvalidAmount)?;

    let core = &mut ctx.accounts.core;
    core.total_claimed_rewards = core.total_claimed_rewards.checked_add(to_be_claimed)
        .ok_or(UniposError::InvalidAmount)?;

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
        bump,
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
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
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
    pub start_time: u64,
    pub lock_period_secs: u64,
    pub locked_rewards: u64,
    pub claimed_rewards: u64,
    pub unstaked: u8,
    pub number: u64,

    pub granted_reward: u64,
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

fn get_unlocked_installment_rewards(now: u64, staker_record: &StakerRecord, installment_num: u64, cliff_period_secs: u64) -> Result<u64> {
    let total_rewards = staker_record.claimed_rewards.checked_add(staker_record.locked_rewards)
        .ok_or(UniposError::InvalidAmount)?;
    let elapsed_time = now.checked_sub(staker_record.start_time)
        .ok_or(UniposError::InvalidAmount)?;
    let unlocked_phase = if elapsed_time <= cliff_period_secs {
        0
    } else if elapsed_time >= staker_record.lock_period_secs {
        installment_num
    } else {
        (elapsed_time - cliff_period_secs).checked_mul(installment_num)
            .ok_or(UniposError::InvalidAmount)?
            .checked_div(staker_record.lock_period_secs)
            .ok_or(UniposError::InvalidAmount)?
    };
    Ok(total_rewards.checked_mul(unlocked_phase)
        .ok_or(UniposError::InvalidAmount)?
        .checked_div(installment_num)
        .ok_or(UniposError::InvalidAmount)?)
}

#[cfg(test)]
mod test {
    use crate::stake::{get_unlocked_installment_rewards, StakerRecord};

    #[test]
    fn test_get_unlocked_installment_rewards() {
        let now = 86400 * 3;
        let staker_record = StakerRecord {
            staker: Default::default(),
            start_time: 86400,
            lock_period_secs: 15552000,
            locked_rewards: 157_000_000_000,
            claimed_rewards: 0,
            unstaked: 0,
            number: 0,
            granted_reward: 0,
            stakeholders: vec![],
            stakeholders_cnt: 0,
        };
        let installment_num = 180;
        let a = get_unlocked_installment_rewards(now, &staker_record, installment_num, 600).unwrap();
        assert_eq!(a, 1_744_444_444);
    }
}