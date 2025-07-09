use anchor_lang::prelude::*;
use anchor_lang::{account, emit, require, AnchorDeserialize, AnchorSerialize, Key};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

mod stake;
mod role;
mod stakeholder;
mod security;

use stake::*;
use role::*;
use stakeholder::*;
use security::*;

declare_id!("E5zfY7jYtr1gXF2yKrAWcxgy1heTefM4VBmgfHMwERxG");

#[program]
pub mod unipos {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        lock_period: u64,
        user_reward_share: u64,
        apy: u64,
        min_stake_amount: u64,
        installment_num: u64,
    ) -> Result<()> {
        let core = &mut ctx.accounts.core;
        core.admin = ctx.accounts.admin.key();
        core.provider = ctx.accounts.provider.key();
        core.mint = ctx.accounts.mint.key();
        core.lock_period_secs = lock_period;
        require!(user_reward_share <= 100, UniposError::InvalidAmount);
        core.user_reward_share = user_reward_share;
        core.apy_percentage = apy;
        core.min_stake_amount = min_stake_amount;
        core.installment_num = installment_num;
        core.total_collateral = 0;
        core.unstaked_collateral = 0;
        core.total_claimed_rewards = 0;
        core.total_security_deposit = 0;
        core.allowed_collateral = 0;
        Ok(())
    }

    pub fn transfer_provider_ownership(ctx: Context<TransferProviderOwnership>) -> Result<()> {
        role::transfer_provider_ownership(ctx)
    }

    pub fn accept_provider_ownership(ctx: Context<AcceptProviderOwnership>) -> Result<()> {
        role::accept_provider_ownership(ctx)
    }

    pub fn init_beneficiary(ctx: Context<InitBeneficiary>) -> Result<()> {
        role::init_beneficiary(ctx)
    }

    pub fn claim_beneficiary_rewards(ctx: Context<ClaimBeneficiaryRewards>) -> Result<()> {
        role::claim_beneficiary_rewards(ctx)
    }

    pub fn stake(ctx: Context<Stake>, number: u64, amount: u64) -> Result<()> {
        stake::stake(ctx, number, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, number: u64) -> Result<()> {
        stake::unstake(ctx, number)
    }

    pub fn add_stakeholder(ctx: Context<AddStakeholder>, number: u64, granted_reward: u64, granted_collateral: u64) -> Result<()> {
        stakeholder::add_stakeholder(ctx, number, granted_reward, granted_collateral)
    }

    pub fn claim_stakeholder_reward(ctx: Context<StakeholderClaim>, number: u64) -> Result<()> {
        stakeholder::claim_stakeholder_reward(ctx, number)
    }

    pub fn claim_stakeholder_collateral(ctx: Context<StakeholderClaim>, number: u64) -> Result<()> {
        stakeholder::claim_stakeholder_collateral(ctx, number)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, number: u64) -> Result<()> {
        stake::claim_rewards(ctx, number)
    }

    pub fn deposit_security(ctx: Context<DepositSecurity>, amount: u64) -> Result<()> {
        security::deposit_security(ctx, amount)
    }

    pub fn withdraw_security(ctx: Context<WithdrawSecurity>, amount: u64) -> Result<()> {
        security::withdraw_security(ctx, amount)
    }

    pub fn collect_from_pool(ctx: Context<CollectFromPool>) -> Result<()> {
        security::collect_from_pool(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 12 * 8 + 5 * 32,
        seeds = [b"core"],
        bump
    )]
    pub core: Account<'info, Core>,

    #[account(
        init,
        payer = admin,
        token::mint = mint,
        token::authority = core,
        seeds = [b"core_vault"],
        bump
    )]
    pub core_vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: NO NEED
    pub provider: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct Core {
    pub admin: Pubkey,
    pub pending_provider: Pubkey,
    pub provider: Pubkey,
    pub mint: Pubkey,
    pub lock_period_secs: u64,
    pub user_reward_share: u64,
    pub apy_percentage: u64,
    pub min_stake_amount: u64,
    pub installment_num: u64,

    pub total_collateral: u64,
    pub unstaked_collateral: u64,
    pub allowed_collateral: u64,

    pub total_claimed_rewards: u64,

    pub total_security_deposit: u64,

    pub beneficiary: Pubkey,
    pub beneficiary_total_rewards: u64,
    pub beneficiary_claimed_rewards: u64,
}

#[error_code]
pub enum UniposError {
    #[msg("Invalid stake number")]
    InvalidStakeNumber,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Amount too small")]
    AmountTooSmall,
    #[msg("Insufficient allowance")]
    InsufficientAllowance,
    #[msg("Not owner")]
    NotOwner,
    #[msg("Lock period not ended")]
    LockPeriodNotEnded,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Not provider")]
    NotProvider,
    #[msg("Insufficient security")]
    InsufficientSecurity,
    #[msg("Not admin")]
    NotAdmin,
    #[msg("Already initialized")]
    AlreadyInitialized,
    #[msg("Invalid address")]
    InvalidAddress,
    #[msg("Not beneficiary")]
    NotBeneficiary,
    #[msg("Too many stakeholders")]
    TooManyStakeholders,
    #[msg("Not a stakeholder")]
    NotStakeholder,
    #[msg("Stakeholder exists")]
    StakeholderExists,
    #[msg("Stakeholder not exists")]
    StakeholderNotExists,
    #[msg("No locked token")]
    NoLockedToken,
    #[msg("Not unstaked")]
    NotUnstaked,
}
