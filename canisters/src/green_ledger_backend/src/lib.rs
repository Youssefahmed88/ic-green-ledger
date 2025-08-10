use candid::{candid_method, Principal};
use ic_cdk_macros::{init, query, update}; // Ensure this is imported from Cargo.toml

// Declare modules
mod errors;
mod types;
mod storage;
mod utils;
mod payout;
mod weather;
mod farmer;
mod nft;
mod policy_manager; // Main policy logic

// Re-export common types and errors for convenience
pub use errors::*;
pub use types::*;

// Public canister methods (exposed via #[update] and #[query])
// These functions act as the public interface, calling into the respective modules.

#[ic_cdk::init]
#[candid_method(init)]
fn init(treasury: Principal) {
    storage::init_state(ic_cdk::caller(), treasury);
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn set_payout_engine(payout_engine: Principal) -> Result<()> {
    policy_manager::set_payout_engine(payout_engine)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn create_policy(
    name: String,
    crop_stage: CropStage,
    threshold: f64,
    premium: u64,
    coverage_amount: u64,
    season: u64,
    season_start: u64,
    season_end: u64,
    subscription_deadline: u64,
    covers_full_season: bool,
    location: String,
    consecutive_dry_days_threshold: u32,
) -> Result<u64> {
    policy_manager::create_policy(
        name,
        crop_stage,
        threshold,
        premium,
        coverage_amount,
        season,
        season_start,
        season_end,
        subscription_deadline,
        covers_full_season,
        location,
        consecutive_dry_days_threshold,
    )
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn subscribe(policy_id: u64) -> Result<()> {
    policy_manager::subscribe(policy_id)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn pause_policy(policy_id: u64) -> Result<()> {
    policy_manager::pause_policy(policy_id)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn resume_policy(policy_id: u64) -> Result<()> {
    policy_manager::resume_policy(policy_id)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn reset_season_state(policy_id: u64) -> Result<()> {
    policy_manager::reset_season_state(policy_id)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_policy_details(policy_id: u64) -> Result<PolicyDetails> {
    policy_manager::get_policy_details(policy_id)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_historical_subscribers(policy_id: u64, season: u64) -> Vec<Principal> {
    policy_manager::get_historical_subscribers(policy_id, season)
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn trigger_automatic_payout(policy_id: u64) -> Result<()> {
    payout::trigger_automatic_payout(policy_id)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn mark_policy_as_payout(policy_id: u64) -> Result<()> {
    payout::mark_policy_as_payout(policy_id)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn set_oracle_canister(oracle_canister: Principal) -> Result<()> {
    weather::set_oracle_canister(oracle_canister)
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn fetch_weather_data(location: String) -> Result<WeatherData> {
    weather::fetch_weather_data(location).await
}

#[ic_cdk::update]
#[candid_method(update)]
pub async fn check_drought_conditions(policy_id: u64) -> Result<bool> {
    weather::check_drought_conditions(policy_id).await
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_weather_data(location: String, from_timestamp: u64) -> Vec<WeatherData> {
    weather::get_weather_data(location, from_timestamp)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn register_farmer(profile: FarmerProfile) -> Result<()> { // Calls the renamed function
    farmer::register_farmer(profile)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_farmer_profile(farmer: Principal) -> Result<FarmerProfile> {
    farmer::get_farmer_profile(farmer)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_farmer_policies(farmer: Principal, season: u64) -> Vec<Subscription> {
    farmer::get_farmer_policies(farmer, season)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_farmer_payouts(farmer: Principal) -> Vec<(String, u64)> { // Updated return type
    farmer::get_farmer_payouts(farmer)
}

#[ic_cdk::update]
#[candid_method(update)]
pub fn mint_nft(policy_id: u64, farmer: Principal) -> Result<NFTMetadata> {
    nft::mint_nft(policy_id, farmer)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_nft_metadata(token_id: u64) -> Result<NFTMetadata> {
    nft::get_nft_metadata(token_id)
}

#[ic_cdk::query]
#[candid_method(query)]
pub fn get_platform_stats() -> PlatformStats {
    storage::get_platform_stats()
}

// Candid generation for testing and interface definition
#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn generate_candid() {
        export_service!();
        let agent_interface = __export_service();
        println!("{}", agent_interface);
    }
}
