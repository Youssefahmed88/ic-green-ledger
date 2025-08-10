use crate::types::*;
use crate::errors::*;
use crate::storage;
use crate::utils;
use candid::Principal;
use ic_cdk::{update, query}; // Explicitly import update and query from ic_cdk

pub fn register_farmer(profile: FarmerProfile) -> Result<()> { // Renamed to `register_farmer`
    let caller = ic_cdk::caller();

    // Validate that caller matches profile wallet
    if caller != profile.wallet_address {
        return Err(PolicyManagerError::Unauthorized);
    }

    storage::save_farmer_profile(caller, profile);
    ic_cdk::println!("FarmerRegistered: farmer={}", caller.to_text()); // Added println for logging
    Ok(())
}

pub fn get_farmer_profile(farmer: Principal) -> Result<FarmerProfile> {
    storage::get_farmer_profile(farmer)
        .ok_or(PolicyManagerError::FarmerNotFound)
}

pub fn get_farmer_policies(farmer: Principal, season: u64) -> Vec<Subscription> {
    let farmer_season_key = utils::get_caller_key(farmer, season);
    storage::get_farmer_policies(&farmer_season_key)
}

pub fn get_farmer_payouts(farmer: Principal) -> Vec<(String, u64)> { // Changed return type to match storage
    let farmer_text = farmer.to_text();
    storage::get_all_payouts_for_farmer(&farmer_text)
}