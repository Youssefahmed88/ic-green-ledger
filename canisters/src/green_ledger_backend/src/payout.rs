use crate::types::*;
use crate::errors::*;
use crate::storage;
use crate::utils;
use ic_cdk::update; // Removed unused `query`
use candid::Principal;

pub fn trigger_automatic_payout(policy_id: u64) -> Result<()> {
    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;

    if policy.status != PolicyStatus::Active {
        return Err(PolicyManagerError::PolicyNotActive);
    }

    if policy.current_subscribers.is_empty() {
        return Err(PolicyManagerError::DivisionByZero);
    }

    // Calculate total payout
    let payout_per_farmer = policy.coverage_amount / policy.current_subscribers.len() as u64;

    // Issue payouts to all subscribers
    for farmer in &policy.current_subscribers {
        let payout_key = format!("{}:{}", farmer.to_text(), policy_id);
        storage::save_payout(payout_key, payout_per_farmer);

        // In production, this would transfer ICP tokens to farmer's wallet
        ic_cdk::println!("Payout issued: farmer={}, amount={}", farmer.to_text(), payout_per_farmer);
    }

    // Update policy status
    policy.status = PolicyStatus::PayoutTriggered;
    storage::save_policy(policy.clone()); // Cloned policy before use in println!

    // Update global stats
    storage::add_to_total_payouts(policy.coverage_amount);

    ic_cdk::println!("Automatic payout triggered for policy {}", policy_id);
    Ok(())
}

pub fn mark_policy_as_payout(policy_id: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    if !utils::is_payout_engine(caller) {
        return Err(PolicyManagerError::Unauthorized);
    }

    let mut policy = storage::get_policy(policy_id)
        .ok_or(PolicyManagerError::PolicyNotFound)?;

    match policy.status {
        PolicyStatus::Active => {},
        _ => return Err(PolicyManagerError::PolicyNotActive),
    }

    policy.status = PolicyStatus::PayoutTriggered;
    storage::save_policy(policy); // No clone needed here as policy is not used after this line

    ic_cdk::println!("PayoutTriggered: policy_id={}", policy_id);
    Ok(())
}
