use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use crate::{Core, UniposError};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::stake::StakerRecord;

pub const MAX_STAKEHOLDERS: u8 = 32;

pub fn add_stakeholder(ctx: Context<AddStakeholder>, number: u64, granted_reward: u64, granted_collateral: u64) -> Result<()> {
	let staker_record = &mut ctx.accounts.staker_record;

	// Check if new grants exceed available amounts
	let new_total_granted_reward = staker_record.granted_reward.checked_add(granted_reward)
		.ok_or(UniposError::InvalidAmount)?;
	let total_available_rewards = staker_record.locked_rewards.checked_add(staker_record.claimed_rewards)
		.ok_or(UniposError::InvalidAmount)?;
	require!(
		new_total_granted_reward <= total_available_rewards,
		UniposError::InsufficientAllowance
	);
	
	let new_total_granted_collateral = staker_record.granted_collateral.checked_add(granted_collateral)
		.ok_or(UniposError::InvalidAmount)?;
	require!(
		new_total_granted_collateral <= staker_record.collateral,
		UniposError::InsufficientAllowance
	);
	require!(staker_record.stakeholders_cnt <= MAX_STAKEHOLDERS, UniposError::InsufficientAllowance);

	let stakeholder = ctx.accounts.stakeholder.key();
	let exists = staker_record.stakeholders.iter().find(|x| x.stakeholder == stakeholder);
	require!(exists.is_none(), UniposError::StakeholderExists);
	// Add new stakeholder
	staker_record.stakeholders.push(StakeholderInfo {
		stakeholder: ctx.accounts.stakeholder.key(),
		granted_reward,
		claimed_reward: 0,
		granted_collateral,
		claimed_collateral: 0,
	});
	staker_record.stakeholders_cnt = staker_record.stakeholders_cnt.checked_add(1)
		.ok_or(UniposError::InvalidAmount)?;
	staker_record.granted_collateral = staker_record.granted_collateral.checked_add(granted_collateral)
		.ok_or(UniposError::InvalidAmount)?;
	staker_record.granted_reward = staker_record.granted_reward.checked_add(granted_reward)
		.ok_or(UniposError::InvalidAmount)?;

	emit!(StakeholderAddedEvent {
		staker: ctx.accounts.staker.key(),
		stakeholder,
		granted_reward,
		granted_collateral,
	});

	Ok(())
}

pub fn claim_stakeholder_reward(ctx: Context<StakeholderClaim>, number: u64) -> Result<()> {
	let stakeholder_key = ctx.accounts.stakeholder.key();
	let staker_record = &mut ctx.accounts.staker_record;

	msg!("before calculate total_rewards");
	let claimed_rewards = staker_record.claimed_rewards;
	let total_rewards = claimed_rewards.checked_add(staker_record.locked_rewards)
		.ok_or(UniposError::InvalidAmount)?;

	// Find the stakeholder in the record
	let mut num: Option<usize> = None;
	for i in 0..staker_record.stakeholders_cnt {
		let info = &mut staker_record.stakeholders[i as usize];
		if info.stakeholder == stakeholder_key {
			num = Some(i as usize);
			break;
		}
	}
	let num = num.ok_or(UniposError::StakeholderNotExists)?;
	let stakeholder_info = &mut staker_record.stakeholders[num];

	msg!("before calculate claimable total reward");
	// Calculate claimable rewards
	let claimable_total_reward = (stakeholder_info.granted_reward as u128 * claimed_rewards as u128).checked_div(total_rewards as u128).ok_or(UniposError::InvalidAmount)? as u64;
	require!(claimable_total_reward > stakeholder_info.claimed_reward, UniposError::NothingToClaim);

	msg!("before calculate claimable_reward");
	let claimable_reward = claimable_total_reward.checked_sub(stakeholder_info.claimed_reward)
		.ok_or(UniposError::InvalidAmount)?;

	// Update claimed rewards
	stakeholder_info.claimed_reward = claimable_total_reward;

	// Transfer rewards to stakeholder
	let transfer_ctx = CpiContext::new(
		ctx.accounts.token_program.to_account_info(),
		Transfer {
			from: ctx.accounts.staker_vault.to_account_info(),
			to: ctx.accounts.stakeholder_token_account.to_account_info(),
			authority: ctx.accounts.core.to_account_info(),
		}
	);
	let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
	token::transfer(transfer_ctx.with_signer(&[pda_sign]), claimable_reward)?;

	emit!(StakeholderRewardClaimedEvent {
		stakeholder: stakeholder_key,
		amount: claimable_reward,
	});

	Ok(())
}

#[derive(Accounts)]
#[instruction(number: u64)]
pub struct AddStakeholder<'info> {
    #[account(
        mut,
        seeds = [b"staker_record", staker.key().as_ref(), number.to_le_bytes().as_ref()],
        bump
    )]
    pub staker_record: Account<'info, StakerRecord>,

    #[account(mut)]
    pub staker: Signer<'info>,

	/// CHECK: no need
	pub stakeholder: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(number: u64)]
pub struct StakeholderClaim<'info> {
    #[account(
        seeds = [b"core"],
        bump
    )]
    pub core: Account<'info, Core>,

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

    /// CHECK: no need
    pub staker: AccountInfo<'info>,

    #[account(mut)]
    pub stakeholder_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stakeholder: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StakeholderInfo {
	pub stakeholder: Pubkey,
	pub granted_reward: u64,
	pub claimed_reward: u64,
	pub granted_collateral: u64,
	pub claimed_collateral: u64,
}

#[event]
pub struct StakeholderAddedEvent {
    pub staker: Pubkey,
    pub stakeholder: Pubkey,
    pub granted_reward: u64,
    pub granted_collateral: u64,
}

#[event]
pub struct StakeholderRewardClaimedEvent {
    pub stakeholder: Pubkey,
    pub amount: u64,
}
