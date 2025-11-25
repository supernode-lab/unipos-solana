use anchor_lang::error_code;

#[error_code]
pub enum SwitchError {
    #[msg("Invalid Parameter")]
    InvalidParameter,

    #[msg("Liquid locking")]
    LiquidLocking,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Insufficient fund")]
    InsufficientFund,

    #[msg("Only admin")]
    OnlyAdmin,

    #[msg("Only provider")]
    OnlyProvider,

    #[msg("forbidden")]
    Forbidden,

    #[msg("StakeInfoMissing")]
    StakeInfoMissing,

    #[msg("Too many providers")]
    TooManyProviders,
    
    #[msg("Too big stakeinfo")]
    TooBigStakeInfo
}
