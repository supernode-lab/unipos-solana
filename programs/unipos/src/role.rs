use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use crate::{Core, UniposError};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::stake::StakerRecord;

pub fn init_beneficiary(ctx: Context<InitBeneficiary>) -> Result<()> {
	let core = &mut ctx.accounts.core;
	require!(core.beneficiary == Pubkey::default(), UniposError::AlreadyInitialized);
	require!(ctx.accounts.beneficiary.key() != Pubkey::default(), UniposError::InvalidAddress);

	core.beneficiary = ctx.accounts.beneficiary.key();
	core.beneficiary_total_rewards = 0;
	core.beneficiary_claimed_rewards = 0;

	emit!(BeneficiaryInitializedEvent {
		beneficiary: ctx.accounts.beneficiary.key(),
	});

	Ok(())
}

pub fn claim_beneficiary_rewards(ctx: Context<ClaimBeneficiaryRewards>) -> Result<()> {
	let core = &mut ctx.accounts.core;
	require!(core.beneficiary == ctx.accounts.beneficiary.key(), UniposError::NotBeneficiary);

	let rewards = core.beneficiary_total_rewards.checked_sub(core.beneficiary_claimed_rewards)
		.ok_or(UniposError::InvalidAmount)?;
	require!(rewards > 0, UniposError::NothingToClaim);

	core.beneficiary_claimed_rewards = core.beneficiary_claimed_rewards.checked_add(rewards)
		.ok_or(UniposError::InvalidAmount)?;
	core.total_claimed_rewards = core.total_claimed_rewards.checked_add(rewards)
		.ok_or(UniposError::InvalidAmount)?;

	// Transfer rewards to beneficiary
	let transfer_ctx = CpiContext::new(
		ctx.accounts.token_program.to_account_info(),
		Transfer {
			from: ctx.accounts.core_vault.to_account_info(),
			to: ctx.accounts.beneficiary_token_account.to_account_info(),
			authority: ctx.accounts.core.to_account_info(),
		}
	);
	let pda_sign: &[&[u8]] = &[b"core", &[ctx.bumps.core]];
	token::transfer(transfer_ctx.with_signer(&[pda_sign]), rewards)?;


	emit!(BeneficiaryRewardsClaimedEvent {
		beneficiary: ctx.accounts.beneficiary.key(),
		amount: rewards,
	});

	Ok(())
}

#[derive(Accounts)]
pub struct InitBeneficiary<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump,
		has_one = admin
    )]
    pub core: Account<'info, Core>,
    pub admin: Signer<'info>,
    /// CHECK: beneficiary address
    pub beneficiary: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimBeneficiaryRewards<'info> {
    #[account(
        mut,
        seeds = [b"core"],
        bump
    )]
    pub core: Account<'info, Core>,
    pub beneficiary: Signer<'info>,
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,
    #[account(
		mut,
		seeds = [b"core_vault"],
		bump,
	)]
    pub core_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct BeneficiaryInitializedEvent {
    pub beneficiary: Pubkey,
}

#[event]
pub struct BeneficiaryRewardsClaimedEvent {
    pub beneficiary: Pubkey,
    pub amount: u64,
}
