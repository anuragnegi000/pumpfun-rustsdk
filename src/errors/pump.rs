use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PumpError {
    #[error("The given account is not authorized to execute this instruction.")]
    NotAuthorized = 0x1770,
    #[error("The program is already initialized.")]
    AlreadyInitialized = 0x1771,
    #[error("slippage: Too much SOL required to buy the given amount of tokens.")]
    TooMuchSolRequired = 0x1772,
    #[error("slippage: Too little SOL received to sell the given amount of tokens.")]
    TooLittleSolReceived = 0x1773,
    #[error("The mint does not match the bonding curve.")]
    MintDoesNotMatchBondingCurve = 0x1774,
    #[error("The bonding curve has completed and liquidity migrated to raydium.")]
    BondingCurveComplete = 0x1775,
    #[error("The bonding curve has not completed.")]
    BondingCurveNotComplete = 0x1776,
    #[error("The program is not initialized.")]
    NotInitialized = 0x1777,
    #[error("Withdraw too frequent")]
    WithdrawTooFrequent = 0x1778,
    #[error("new_size should be > current_size")]
    NewSizeShouldBeGreaterThanCurrentSize = 0x1779,
    #[error("Account type not supported")]
    AccountTypeNotSupported = 0x177A,
    #[error("initial_real_token_reserves should be less than token_total_supply")]
    InitialRealTokenReservesShouldBeLessThanTokenTotalSupply = 0x177B,
    #[error("initial_virtual_token_reserves should be greater than initial_real_token_reserves")]
    InitialVirtualTokenReservesShouldBeGreaterThanInitialRealTokenReserves = 0x177C,
    #[error("fee_basis_points greater than maximum")]
    FeeBasisPointsGreaterThanMaximum = 0x177D,
    #[error("Withdraw authority cannot be set to System Program ID")]
    AllZerosWithdrawAuthority = 0x177E,
    #[error("pool_migration_fee should be less than final_real_sol_reserves")]
    PoolMigrationFeeShouldBeLessThanFinalRealSolReserves = 0x177F,
    #[error("pool_migration_fee should be greater than creator_fee + MAX_MIGRATE_FEES")]
    PoolMigrationFeeShouldBeGreaterThanCreatorFeePlusMaxMigrateFees = 0x1780,
    #[error("Migrate instruction is disabled")]
    DisabledWithdraw = 0x1781,
    #[error("Migrate instruction is disabled")]
    DisabledMigrate = 0x1782,
    #[error("Invalid creator pubkey")]
    InvalidCreator = 0x1783,
    #[error("Buy zero amount")]
    BuyZeroAmount = 0x1784,
    #[error("Not enough tokens to buy")]
    NotEnoughTokensToBuy = 0x1785,
    #[error("Sell zero amount")]
    SellZeroAmount = 0x1786,
    #[error("Not enough tokens to sell")]
    NotEnoughTokensToSell = 0x1787,
    #[error("Overflow")]
    Overflow = 0x1788,
    #[error("Truncation")]
    Truncation = 0x1789,
    #[error("Division by zero")]
    DivisionByZero = 0x178A,
    #[error("Not enough remaining accounts")]
    NotEnoughRemainingAccounts = 0x178B,
    #[error("All fee recipients should be non-zero")]
    AllFeeRecipientsShouldBeNonZero = 0x178C,
    #[error("Unsorted or not unique fee recipients")]
    UnsortedNotUniqueFeeRecipients = 0x178D,
    #[error("Creator should not be zero")]
    CreatorShouldNotBeZero = 0x178E,
    #[error("Start time in the past")]
    StartTimeInThePast = 0x178F,
    #[error("End time in the past")]
    EndTimeInThePast = 0x1790,
    #[error("End time before start time")]
    EndTimeBeforeStartTime = 0x1791,
    #[error("Time range too large")]
    TimeRangeTooLarge = 0x1792,
    #[error("End time before current day")]
    EndTimeBeforeCurrentDay = 0x1793,
    #[error("Supply update for finished range")]
    SupplyUpdateForFinishedRange = 0x1794,
    #[error("Day index after end index")]
    DayIndexAfterEndIndex = 0x1795,
    #[error("Day in active range")]
    DayInActiveRange = 0x1796,
    #[error("Invalid incentive mint")]
    InvalidIncentiveMint = 0x1797,
    #[error("Buy: Not enough SOL to cover for rent exemption.")]
    BuyNotEnoughSolToCoverRent = 0x1798,
    #[error("Buy: Not enough SOL to cover for fees.")]
    BuyNotEnoughSolToCoverFees = 0x1799,
    #[error("Slippage: Would buy less tokens than expected min_tokens_out")]
    BuySlippageBelowMinTokensOut = 0x179A,
    #[error("Name too long")]
    NameTooLong = 0x179B,
    #[error("Symbol too long")]
    SymbolTooLong = 0x179C,
    #[error("Uri too long")]
    UriTooLong = 0x179D,
    #[error("CreateV2 disabled")]
    CreateV2Disabled = 0x179E,
    #[error("CPI initialize mayhem failed")]
    CpitializeMayhemFailed = 0x179F,
    #[error("Mayhem mode disabled")]
    MayhemModeDisabled = 0x17A0,
    #[error("creator has been migrated to sharing config, use pump_fees::reset_fee_sharing_config instead")]
    CreatorMigratedToSharingConfig = 0x17A1,
    #[error("creator_vault has been migrated to sharing config, use pump:distribute_creator_fees instead")]
    UnableToDistributeCreatorVaultMigratedToSharingConfig = 0x17A2,
    #[error("Sharing config is not active")]
    SharingConfigNotActive = 0x17A3,
    #[error("The recipient account is executable, so it cannot receive lamports, remove it from the team first")]
    UnableToDistributeCreatorFeesToExecutableRecipient = 0x17A4,
    #[error("Bonding curve creator does not match sharing config")]
    BondingCurveAndSharingConfigCreatorMismatch = 0x17A5,
    #[error("Remaining accounts do not match shareholders, make sure to pass exactly the same pubkeys in the same order")]
    ShareholdersAndRemainingAccountsMismatch = 0x17A6,
    #[error("Share bps must be greater than 0")]
    InvalidShareBps = 0x17A7,
    #[error("Cashback is not enabled")]
    CashbackNotEnabled = 0x17A8,
}

impl From<PumpError> for solana_program_error::ProgramError {
    fn from(e: PumpError) -> Self {
        solana_program_error::ProgramError::Custom(e as u32)
    }
}
