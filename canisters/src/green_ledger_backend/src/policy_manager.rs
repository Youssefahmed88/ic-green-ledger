use crate::types::*;
use crate::errors::*;
use crate::storage;
use crate::utils;
use ic_cdk::api::time;
use candid::Principal;

#[ic_cdk::update]
pub fn set_payout_engine(payout_engine: Principal) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }
    
    storage::set_payout_engine(payout_engine);
    Ok(())
}

#[ic_cdk::update]
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
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }
    
    let policy_id = storage::increment_next_policy_id();
    
    let policy = Policy {
        id: policy_id,
        name: name.clone(),
        crop_stage,
        trigger_threshold: threshold,
        premium,
        coverage_amount,
        season,
        season_start,
        season_end,
        subscription_deadline,
        covers_full_season,
        status: PolicyStatus::Active,
        current_subscribers: Vec::new(),
        last_subscribed_season: std::collections::HashMap::new(),
        location,
        consecutive_dry_days_threshold,
        last_weather_check: 0,
        nft_metadata: Vec::new(),
    };
    
    storage::save_policy(policy);
    
    ic_cdk::println!("PolicyCreated: id={}, name={}, season={}", policy_id, name, season);
    Ok(policy_id)
}

#[ic_cdk::update]
pub fn subscribe(policy_id: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    let current_time = time();
    
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;
    
    utils::validate_policy_subscription(&policy, caller, current_time)?;
    
    let caller_season_key = utils::get_caller_key(caller, policy.season);
    
    // Check full season coverage constraints
    if policy.covers_full_season {
        let has_full_cover = storage::get_farmer_season_full_cover(&caller_season_key);
        if has_full_cover {
            return Err(PolicyManagerError::FullSeasonCoverageExists);
        }
        storage::set_farmer_season_full_cover(caller_season_key.clone(), true);
    } else {
        let has_full_cover = storage::get_farmer_season_full_cover(&caller_season_key);
        if has_full_cover {
            return Err(PolicyManagerError::CannotSubscribeAfterFullSeason);
        }
    }
    
    // Update policy subscription data
    policy.last_subscribed_season.insert(caller, policy.season);
    policy.current_subscribers.push(caller);
    
    // Update farmer policies
    let subscription = Subscription {
        policy_id,
        timestamp: current_time,
    };
    
    let mut subscriptions = storage::get_farmer_policies(&caller_season_key);
    subscriptions.push(subscription);
    storage::save_farmer_policies(caller_season_key, subscriptions);
    
    storage::save_policy(policy);
    
    ic_cdk::println!("Subscribed: farmer={}, policy_id={}, season={}", 
        caller.to_text(), policy_id, policy.season);
    Ok(())
}

#[ic_cdk::update]
pub fn pause_policy(policy_id: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }
    
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;
    
    policy.status = PolicyStatus::Paused;
    storage::save_policy(policy);
    
    ic_cdk::println!("PolicyPaused: policy_id={}", policy_id);
    Ok(())
}

#[ic_cdk::update]
pub fn resume_policy(policy_id: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }
    
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;
    
    policy.status = PolicyStatus::Active;
    storage::save_policy(policy);
    
    ic_cdk::println!("PolicyResumed: policy_id={}", policy_id);
    Ok(())
}

#[ic_cdk::update]
pub fn reset_season_state(policy_id: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_owner(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }
    
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;
    
    match policy.status {
        PolicyStatus::PayoutTriggered | PolicyStatus::Paused => {},
        _ => return Err(PolicyManagerError::PolicyNotCompletedOrPaused),
    }
    
    // Store historical subscribers
    let policy_season_key = utils::get_policy_season_key(policy_id, policy.season);
    storage::save_historical_subscribers(policy_season_key, policy.current_subscribers.clone());
    
    // Reset policy for new season
    policy.current_subscribers.clear();
    policy.last_subscribed_season.insert(caller, policy.season);
    policy.season += 1;
    policy.status = PolicyStatus::Active;
    
    storage::save_policy(policy);
    
    ic_cdk::println!("SeasonReset: policy_id={}, new_season={}", policy_id, policy.season);
    Ok(())
}

#[ic_cdk::query]
pub fn get_policy_details(policy_id: u64) -> Result<PolicyDetails> {
    let policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;
    
    Ok(PolicyDetails {
        name: policy.name,
        threshold: policy.trigger_threshold,
        premium: policy.premium,
        coverage_amount: policy.coverage_amount,
        status: policy.status,
        season: policy.season,
        season_start: policy.season_start,
        season_end: policy.season_end,
        subscription_deadline: policy.subscription_deadline,
        covers_full_season: policy.covers_full_season,
        subscriber_count: policy.current_subscribers.len(),
        crop_stage: policy.crop_stage,
    })
}

#[ic_cdk::query]
pub fn get_historical_subscribers(policy_id: u64, season: u64) -> Vec<Principal> {
    let policy_season_key = utils::get_policy_season_key(policy_id, season);
    storage::get_historical_subscribers(&policy_season_key)
}
