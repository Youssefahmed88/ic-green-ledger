use crate::types::*;
use crate::errors::*;
use crate::storage;
use crate::utils;
use ic_cdk::api::time;
use ic_cdk::{update, query};
use candid::Principal; // Added Principal import

pub fn set_oracle_canister(oracle_canister: Principal) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }

    storage::set_oracle_canister(oracle_canister);
    Ok(())
}

pub async fn fetch_weather_data(location: String) -> Result<WeatherData> {
    let oracle_canister = storage::get_oracle_canister() // Now correctly calls the new storage function
        .ok_or(PolicyManagerError::OracleNotSet)?;

    // Mock weather data for demonstration
    // In production, this would call the oracle canister
    let weather_data = WeatherData {
        rainfall_mm: 2.5,
        temperature_celsius: 28.0,
        humidity_percent: 65.0,
        timestamp: time(),
        location: location.clone(),
    };

    let weather_key = format!("{}:{}", location, time());
    storage::save_weather_data(weather_key, weather_data.clone());

    Ok(weather_data)
}

pub async fn check_drought_conditions(policy_id: u64) -> Result<bool> {
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;

    let weather_data = fetch_weather_data(policy.location.clone()).await?;
    let is_drought = weather_data.rainfall_mm < policy.trigger_threshold;

    if is_drought {
        let recent_weather = get_recent_weather_data(&policy.location, 7)?;
        let consecutive_dry_days = utils::count_consecutive_dry_days(&recent_weather, policy.trigger_threshold);

        if consecutive_dry_days >= policy.consecutive_dry_days_threshold {
            crate::payout::trigger_automatic_payout(policy_id)?;
            return Ok(true);
        }
    }

    policy.last_weather_check = time();
    storage::save_policy(policy);

    Ok(is_drought)
}

pub fn get_weather_data(location: String, from_timestamp: u64) -> Vec<WeatherData> {
    storage::get_all_weather_data_for_location(&location, from_timestamp)
}

fn get_recent_weather_data(location: &str, days: u32) -> Result<Vec<WeatherData>> {
    let current_time = time();
    let days_in_nanos = days as u64 * 24 * 60 * 60 * 1_000_000_000;
    let from_timestamp = current_time.saturating_sub(days_in_nanos);

    Ok(storage::get_all_weather_data_for_location(location, from_timestamp))
}
