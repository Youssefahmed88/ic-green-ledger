// src/errors.rs
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Debug)]
pub enum PolicyManagerError {
    Unauthorized,
    PolicyNotFound,
    PolicyNotActive,
    SubscriptionDeadlinePassed,
    IncorrectPremium,
    AlreadySubscribed,
    FullSeasonCoverageExists,
    CannotSubscribeAfterFullSeason,
    PolicyNotCompletedOrPaused,
    InsufficientBalance,
    OracleNotSet,
    FarmerNotFound,
    WeatherDataNotAvailable,
    InvalidCropStage,
    DivisionByZero,
    StorageError,
}

pub type Result<T> = std::result::Result<T, PolicyManagerError>;