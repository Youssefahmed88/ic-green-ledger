use std::cell::RefCell;
use std::collections::HashMap;

use candid::Principal;

use crate::types::{Policy, State, WeatherData, Subscription, FarmerProfile, PlatformStats};

// Thread-local storage for the canister's state.
// RefCell provides interior mutability, allowing us to modify the state
// within immutable contexts (like `with` closures).
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static POLICIES: RefCell<HashMap<u64, Policy>> = RefCell::new(HashMap::new());
    static PAYOUTS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static FARMER_POLICIES: RefCell<HashMap<String, Vec<Subscription>>> = RefCell::new(HashMap::new());
    static FARMER_SEASON_FULL_COVER: RefCell<HashMap<String, bool>> = RefCell::new(HashMap::new());
    static HISTORICAL_SUBSCRIBERS: RefCell<HashMap<String, Vec<Principal>>> = RefCell::new(HashMap::new());
    static WEATHER_DATA_BY_LOCATION: RefCell<HashMap<String, Vec<WeatherData>>> = RefCell::new(HashMap::new());
    static FARMER_PROFILES: RefCell<HashMap<Principal, FarmerProfile>> = RefCell::new(HashMap::new()); // Added for farmer profiles
}

// --- State Management ---

/// Returns a clone of the current global state.
pub fn get_state() -> State {
    STATE.with(|s| s.borrow().clone())
}

/// Sets the owner principal in the global state.
pub fn set_owner(owner: Principal) {
    STATE.with(|s| s.borrow_mut().owner = Some(owner));
}

/// Sets the treasury principal in the global state.
pub fn set_treasury(treasury: Principal) {
    STATE.with(|s| s.borrow_mut().treasury = Some(treasury));
}

/// Sets the payout engine principal in the global state.
pub fn set_payout_engine(payout_engine: Principal) {
    STATE.with(|s| s.borrow_mut().payout_engine = Some(payout_engine));
}

/// Sets the oracle canister principal in the global state.
pub fn set_oracle_canister(oracle_canister: Principal) {
    STATE.with(|s| s.borrow_mut().oracle_canister = Some(oracle_canister));
}

/// Retrieves the oracle canister principal from the global state.
pub fn get_oracle_canister() -> Option<Principal> {
    STATE.with(|s| s.borrow().oracle_canister)
}

/// Increments the next available policy ID and returns the new ID.
pub fn increment_next_policy_id() -> u64 {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.next_policy_id += 1;
        state.next_policy_id
    })
}

/// Increments the next available NFT ID and returns the new ID.
pub fn increment_next_nft_id() -> u64 {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.next_nft_id += 1;
        state.next_nft_id
    })
}

/// Adds the given amount to the total payouts issued.
pub fn add_to_total_payouts(amount: u64) {
    STATE.with(|s| s.borrow_mut().total_payouts_issued += amount);
}

/// Increments the count of active policies.
pub fn increment_active_policies_count() {
    STATE.with(|s| s.borrow_mut().active_policies_count += 1);
}

/// Decrements the count of active policies, saturating at 0.
pub fn decrement_active_policies_count() {
    STATE.with(|s| s.borrow_mut().active_policies_count = s.borrow().active_policies_count.saturating_sub(1));
}

/// Initializes the canister's state with owner and treasury.
pub fn init_state(owner: Principal, treasury: Principal) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.owner = Some(owner);
        state.treasury = Some(treasury);
    });
}

/// Returns platform-wide statistics.
pub fn get_platform_stats() -> PlatformStats {
    STATE.with(|s| {
        let state = s.borrow();
        PlatformStats {
            total_payouts_issued: state.total_payouts_issued,
            active_policies_count: state.active_policies_count,
            total_policies_created: state.next_policy_id.saturating_sub(1), // Assuming IDs start from 1
            total_nfts_minted: state.next_nft_id.saturating_sub(1), // Assuming IDs start from 1
        }
    })
}

// --- Policy Management ---

/// Saves or updates a policy in storage.
pub fn save_policy(policy: Policy) {
    POLICIES.with(|p| p.borrow_mut().insert(policy.id, policy));
}

/// Retrieves a policy by its ID.
pub fn get_policy(policy_id: u64) -> Option<Policy> {
    POLICIES.with(|p| p.borrow().get(&policy_id).cloned())
}

/// Deletes a policy by its ID.
pub fn delete_policy(policy_id: u64) {
    POLICIES.with(|p| p.borrow_mut().remove(&policy_id));
}

// --- Farmer Profile Management ---

/// Saves a farmer's profile.
pub fn save_farmer_profile(principal: Principal, profile: FarmerProfile) {
    FARMER_PROFILES.with(|fp| fp.borrow_mut().insert(principal, profile));
}

/// Retrieves a farmer's profile by their principal.
pub fn get_farmer_profile(principal: Principal) -> Option<FarmerProfile> {
    FARMER_PROFILES.with(|fp| fp.borrow().get(&principal).cloned())
}

// --- Subscription/Farmer Data ---

/// Saves a payout record. The `payout_key` is typically "farmer_principal:policy_id".
pub fn save_payout(payout_key: String, amount: u64) {
    PAYOUTS.with(|p| p.borrow_mut().insert(payout_key, amount));
}

/// Retrieves all payout amounts for a given farmer.
pub fn get_all_payouts_for_farmer(farmer_principal_text: &str) -> Vec<(String, u64)> { // Changed return type to match farmer.rs
    PAYOUTS.with(|p| {
        p.borrow()
            .iter()
            .filter_map(|(key, &amount)| {
                if key.starts_with(farmer_principal_text) {
                    Some((key.clone(), amount)) // Return (key, amount)
                } else {
                    None
                }
            })
            .collect()
    })
}

/// Retrieves a farmer's subscriptions for a specific season.
/// The `caller_season_key` is typically "farmer_principal:season".
pub fn get_farmer_policies(caller_season_key: &str) -> Vec<Subscription> {
    FARMER_POLICIES.with(|fp| fp.borrow().get(caller_season_key).cloned().unwrap_or_default())
}

/// Saves a farmer's subscriptions for a specific season.
/// The `caller_season_key` is typically "farmer_principal:season".
pub fn save_farmer_policies(caller_season_key: String, subscriptions: Vec<Subscription>) {
    FARMER_POLICIES.with(|fp| fp.borrow_mut().insert(caller_season_key, subscriptions));
}

/// Checks if a farmer has full season coverage for a given season.
/// The `caller_season_key` is typically "farmer_principal:season".
pub fn get_farmer_season_full_cover(caller_season_key: &str) -> bool {
    FARMER_SEASON_FULL_COVER.with(|fsfc| fsfc.borrow().get(caller_season_key).cloned().unwrap_or(false))
}

/// Sets the full season coverage status for a farmer for a given season.
/// The `caller_season_key` is typically "farmer_principal:season".
pub fn set_farmer_season_full_cover(caller_season_key: String, has_full_cover: bool) {
    FARMER_SEASON_FULL_COVER.with(|fsfc| fsfc.borrow_mut().insert(caller_season_key, has_full_cover));
}

/// Saves historical subscribers for a specific policy and season.
/// The `policy_season_key` is typically "policy_id:season".
pub fn save_historical_subscribers(policy_season_key: String, subscribers: Vec<Principal>) {
    HISTORICAL_SUBSCRIBERS.with(|hs| hs.borrow_mut().insert(policy_season_key, subscribers));
}

/// Retrieves historical subscribers for a specific policy and season.
/// The `policy_season_key` is typically "policy_id:season".
pub fn get_historical_subscribers(policy_season_key: &str) -> Vec<Principal> {
    HISTORICAL_SUBSCRIBERS.with(|hs| hs.borrow().get(policy_season_key).cloned().unwrap_or_default())
}

// --- Weather Data ---

/// Saves weather data. The `weather_key` is expected to be "location:timestamp".
/// The data is stored grouped by location.
pub fn save_weather_data(weather_key: String, data: WeatherData) {
    let parts: Vec<&str> = weather_key.split(':').collect();
    if let Some(location) = parts.first() {
        WEATHER_DATA_BY_LOCATION.with(|w| {
            let mut weather_map = w.borrow_mut();
            let entry = weather_map.entry(location.to_string()).or_insert_with(Vec::new);
            entry.push(data);
            entry.sort_by_key(|d| d.timestamp);
        });
    }
}

/// Retrieves all weather data for a given location from a specified timestamp onwards.
pub fn get_all_weather_data_for_location(location: &str, from_timestamp: u64) -> Vec<WeatherData> {
    WEATHER_DATA_BY_LOCATION.with(|w| {
        w.borrow()
            .get(location)
            .map(|data_vec| {
                data_vec
                    .iter()
                    .filter(|data| data.timestamp >= from_timestamp)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    })
}
