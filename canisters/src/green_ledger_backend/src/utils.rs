// src/utils.rs
use crate::types::*;
use crate::errors::*;
use candid::Principal;

pub fn get_caller_key(caller: Principal, season: u64) -> String {
    format!("{}:{}", caller.to_text(), season)
}

pub fn get_policy_season_key(policy_id: u64, season: u64) -> String {
    format!("{}:{}", policy_id, season)
}

pub fn is_owner(caller: Principal) -> bool {
    crate::storage::get_state().owner.map_or(false, |owner| owner == caller)
}

pub fn is_payout_engine(caller: Principal) -> bool {
    crate::storage::get_state().payout_engine.map_or(false, |engine| engine == caller)
}

pub fn count_consecutive_dry_days(weather_data: &[WeatherData], threshold: f64) -> u32 {
    let mut consecutive_days = 0;
    let mut max_consecutive = 0;
    
    for data in weather_data {
        if data.rainfall_mm < threshold {
            consecutive_days += 1;
            max_consecutive = max_consecutive.max(consecutive_days);
        } else {
            consecutive_days = 0;
        }
    }
    
    max_consecutive
}

pub fn validate_policy_subscription(policy: &Policy, caller: Principal, current_time: u64) -> Result<()> {
    // Validate policy conditions
    match policy.status {
        PolicyStatus::Active => {},
        _ => return Err(PolicyManagerError::PolicyNotActive),
    }
    
    if current_time > policy.subscription_deadline {
        return Err(PolicyManagerError::SubscriptionDeadlinePassed);
    }
    
    // Check if already subscribed this season
    if let Some(&last_season) = policy.last_subscribed_season.get(&caller) {
        if last_season >= policy.season {
            return Err(PolicyManagerError::AlreadySubscribed);
        }
    }
    
    Ok(())
}