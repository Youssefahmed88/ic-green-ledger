use candid::{CandidType, Deserialize, Principal};
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)] // Added PartialEq
pub enum PolicyStatus {
    Active,
    Paused,
    PayoutTriggered,
    Completed,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum CropStage {
    FloweringStage,
    GrainFillingStage,
    FullSeason,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WeatherData {
    pub rainfall_mm: f64,
    pub temperature_celsius: f64,
    pub humidity_percent: f64,
    pub timestamp: u64,
    pub location: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct FarmerProfile {
    pub name: String,
    pub farm_size_hectares: f64,
    pub location: String,
    pub phone_number: Option<String>,
    pub national_id: String,
    pub wallet_address: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct NFTMetadata {
    pub token_id: u64,
    pub farmer_profile: FarmerProfile,
    pub policy_details: String,
    pub ipfs_hash: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Policy {
    pub id: u64,
    pub name: String,
    pub crop_stage: CropStage,
    pub trigger_threshold: f64,
    pub premium: u64,
    pub coverage_amount: u64,
    pub season: u64,
    pub season_start: u64,
    pub season_end: u64,
    pub subscription_deadline: u64,
    pub covers_full_season: bool,
    pub status: PolicyStatus,
    pub current_subscribers: Vec<Principal>,
    pub last_subscribed_season: HashMap<Principal, u64>,
    pub location: String,
    pub consecutive_dry_days_threshold: u32,
    pub last_weather_check: u64,
    pub nft_metadata: Vec<NFTMetadata>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Subscription {
    pub policy_id: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct State {
    pub owner: Option<Principal>,
    pub treasury: Option<Principal>,
    pub payout_engine: Option<Principal>,
    pub oracle_canister: Option<Principal>,
    pub next_policy_id: u64,
    pub next_nft_id: u64,
    pub total_payouts_issued: u64,
    pub active_policies_count: u64,
}

#[derive(CandidType, Deserialize)]
pub struct PolicyDetails {
    pub name: String,
    pub threshold: f64,
    pub premium: u64,
    pub coverage_amount: u64,
    pub status: PolicyStatus,
    pub season: u64,
    pub season_start: u64,
    pub season_end: u64,
    pub subscription_deadline: u64,
    pub covers_full_season: bool,
    pub subscriber_count: usize,
    pub(crate) crop_stage: CropStage,
}

#[derive(CandidType, Deserialize)]
pub struct PlatformStats {
    pub total_payouts_issued: u64,
    pub active_policies_count: u64,
    pub total_policies_created: u64,
    pub total_nfts_minted: u64,
}
