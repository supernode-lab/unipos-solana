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

	let rewards = core.beneficiary_total_rewards - core.beneficiary_claimed_rewards;
	require!(rewards > 0, UniposError::NothingToClaim);

	core.beneficiary_claimed_rewards += rewards;
	core.total_claimed_rewards += rewards;

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

pub fn transfer_provider_ownership(ctx: Context<TransferProviderOwnership>) -> Result<()> {
	let core = &mut ctx.accounts.core;
	core.pending_provider = ctx.accounts.new_provider.key();
	emit!(OwnershipTransferredEvent{
		old: core.provider.key(),
		new: core.pending_provider.key(),
	});
	Ok(())
}

pub fn accept_provider_ownership(ctx: Context<AcceptProviderOwnership>) -> Result<()> {
	let core = &mut ctx.accounts.core;
	let old = core.provider;
	core.provider = core.pending_provider;
	core.pending_provider = Pubkey::default();
	emit!(OwnershipTransferAcceptedEvent{
		old,
		new: core.provider.key(),
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

#[derive(Accounts)]
pub struct TransferProviderOwnership<'info> {
	#[account(
        mut,
        seeds = [b"core"],
        bump,
		has_one = provider,
	)]
	pub core: Account<'info, Core>,

	/// CHECK: no need
	pub new_provider: AccountInfo<'info>,

	pub provider: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptProviderOwnership<'info> {
	#[account(
        mut,
        seeds = [b"core"],
        bump,
		has_one = pending_provider
	)]
	pub core: Account<'info, Core>,

	pub pending_provider: Signer<'info>,
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

#[event]
pub struct OwnershipTransferredEvent {
	pub old: Pubkey,
	pub new: Pubkey,
}

#[event]
pub struct OwnershipTransferAcceptedEvent {
	pub old: Pubkey,
	pub new: Pubkey,
}
