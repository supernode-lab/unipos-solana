use crate::errors::SwitchError;
use anchor_lang::prelude::*;
use common::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub providers: Vec<Pubkey>,
    pub beneficiary: Pubkey,
    pub token_mint: Pubkey,
    pub usdt_mint: Pubkey,
    pub usdt_subscription_enabled: bool,
    pub token_subscription_enabled: bool,
    pub liquid_lock_period: u64,
    pub min_subscribe_amount: u64,
}

impl Config {
    pub const LEN: usize =
        8 + 32 + (4 + 32 * Config::PROVIDERS_MAX_NUM) + 32 + 32 + 32 + 1 + 1 + 8 + 8;
    pub const SEED: &'static [u8] = "config".as_bytes();
    pub const PROVIDERS_MAX_NUM: usize = 8;

    pub fn only_admin(&self, pubkey: Pubkey) -> Result<()> {
        if self.admin == pubkey {
            Ok(())
        } else {
            Err(SwitchError::OnlyAdmin)?
        }
    }

    pub fn is_provider(&self, pubkey: Pubkey) -> bool {
        self.providers.contains(&pubkey)
    }

    pub fn only_provider(&self, pubkey: Pubkey) -> Result<()> {
        if self.is_provider(pubkey) {
            Ok(())
        } else {
            Err(SwitchError::OnlyProvider)?
        }
    }

    pub fn is_usdt_subscription_enabled(&self) -> Result<()> {
        if self.usdt_subscription_enabled {
            Ok(())
        } else {
            Err(SwitchError::Forbidden)?
        }
    }

    pub fn is_token_subscription_enabled(&self) -> Result<()> {
        if self.token_subscription_enabled {
            Ok(())
        } else {
            Err(SwitchError::Forbidden)?
        }
    }

    pub fn add_provider(&mut self, pubkey: Pubkey) -> Result<()> {
        if self.is_provider(pubkey) {
            return Ok(());
        }

        if self.providers.len() >= Config::PROVIDERS_MAX_NUM {
            Err(SwitchError::TooManyProviders)?;
        }

        self.providers.push(pubkey);
        Ok(())
    }

    pub fn remove_provider(&mut self, pubkey: Pubkey) -> Result<()> {
        if let Some(pos) = self.providers.iter().position(|x| x == &pubkey) {
            self.providers.remove(pos);
        };

        Ok(())
    }
}

#[account]
pub struct AssetInfo {
    pub last_deposit_time: u64,
    pub total_liquid: u64,
    pub deposited: u64,
    pub exchanged: u64,
    pub total_usdt: u64,
    pub withdrawn_usdt: u64,
}

impl AssetInfo {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 8;
    pub const SEED: &'static [u8] = "asset_info".as_bytes();

    pub fn available_token(&self) -> u64 {
        let used = self.deposited + self.exchanged;
        if self.total_liquid > used {
            self.total_liquid - used
        } else {
            0
        }
    }

    pub fn balance_usdt(&self) -> u64 {
        self.total_usdt - self.withdrawn_usdt
    }
}

#[account]
pub struct PriceInfo {
    pub price: u128,
    pub token_decimals: u8,
    pub usdt_decimals: u8,
}

impl PriceInfo {
    pub const LEN: usize = 8 + 16 + 1 + 1;
    pub const SEED: &'static [u8] = "price_info".as_bytes();

    pub fn decimals(&self) -> u8 {
        self.token_decimals - self.usdt_decimals + FLOAT_DECIMALS
    }
    pub fn calc_token(&self, usdt: u64) -> u64 {
        ((usdt as u128) * (10u128.pow(self.decimals() as u32)) / self.price) as u64
    }

    pub fn cal_usdt(&self, token: u64) -> u64 {
        ((token as u128) * self.price / (10u128.pow(self.decimals() as u32))) as u64
    }
}

#[account]
pub struct StakeInfo {
    pub stakes: [Pubkey; 8],
    pub ratios: [u128; 8],
    pub kpi_stake: Pubkey,
    pub kpi_ratio: u128,
}

impl StakeInfo {
    pub const LEN: usize = 8 + (32 * 8) + (16 * 8) + 32 + 16;
    pub const SEED: &'static [u8] = "stake_info".as_bytes();

    pub fn stake_num(&self) -> usize {
        for i in 0..8 {
            if self.ratios[0] == 0 {
                return i;
            }
        }

        8
    }

    pub fn set(
        &mut self,
        stakecores: &[Pubkey],
        ratios: &[u128],
        kpi_stakecore: Pubkey,
        kpi_ratio: u128,
    ) -> Result<()> {
        if stakecores.len() != ratios.len() {
            Err(SwitchError::InvalidParameter)?
        }

        if stakecores.len() >= 8 {
            Err(SwitchError::TooBigStakeInfo)?
        }

        let total_ratio: u128 = ratios.iter().sum();
        if total_ratio != PRECISION {
            Err(SwitchError::InvalidParameter)?
        }

        for i in 0..ratios.len() {
            self.ratios[i] = ratios[i];
            self.stakes[i] = stakecores[i];
        }

        for i in ratios.len()..8 {
            self.ratios[i] = 0;
        }

        self.kpi_stake = kpi_stakecore;
        self.kpi_ratio = kpi_ratio;

        Ok(())
    }
}

pub const TOKEN_VAULT_SEED: &'static [u8] = b"token_vault";
pub const USDT_VAULT_SEED: &'static [u8] = b"usdt_vault";

pub const VAULT_AUTHORITY_SEED: &'static [u8] = b"vault_authority";
