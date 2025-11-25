use anchor_lang::prelude::*;

#[event]
pub struct AdminUpdated {
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct ProviderAdded {
    pub account: Pubkey,
    pub new_provider: Pubkey,
}

#[event]
pub struct ProviderRemoved {
    pub account: Pubkey,
    pub provider: Pubkey,
}

#[event]
pub struct BeneficiaryUpdated {
    pub new_beneficiary: Pubkey,
}

#[event]
pub struct LiquidLockPeriodUpdated {
    pub old_liquid_lock_period: u64,
    pub new_liquid_lock_period: u64,
}

#[event]
pub struct MinSubscribeAmountUpdated {
    pub old_min_subscribe_amount: u64,
    pub new_min_subscribe_amount: u64,
}

#[event]
pub struct SubscriptionsUpdated {
    pub usdt_enabled: bool,
    pub token_enabled: bool,
}

#[event]
pub struct PriceUpdated {
    pub old_price: u128,
    pub new_price: u128,
}

#[event]
pub struct StakeInfoUpdated {
    pub stakecores: Vec<Pubkey>,
    pub ratios: Vec<u128>,
    pub kpi_stakecore: Pubkey,
    pub kpi_ratio: u128,
}

#[event]
pub struct UsdtWithdrawn {
    pub account: Pubkey,
    pub usdt_account: Pubkey,
    pub amount: u64,
}

#[event]
pub struct LiquidDeposited {
    pub account: Pubkey,
    pub amount: u64,
    pub total_liquid: u64,
}

#[event]
pub struct LiquidWithdrawn {
    pub account: Pubkey,
    pub token_account: Pubkey,
    pub amount: u64,
    pub remaining_liquid: u64,
}

#[event]
pub struct SubscribedByUSDT {
    pub account: Pubkey,
    pub usdt_amount: u64,
    pub price: u128,
    pub owners: Vec<Pubkey>,
    pub numbers: Vec<u64>,
    pub token_amount: u64,
    pub kpi_number: u64,
    pub kpi_amount: u64,
}

#[event]
pub struct SubscribedByToken {
    pub account: Pubkey,
    pub owners: Vec<Pubkey>,
    pub numbers: Vec<u64>,
    pub amount: u64,
    pub kpi_number: u64,
    pub kpi_amount: u64,
}
