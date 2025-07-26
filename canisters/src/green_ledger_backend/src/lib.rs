// src/lib.rs

mod types;
mod storage;
//mod weather;
mod policy_manager;
//mod farmer;
//mod payout;
//mod nft;
mod utils;
mod errors;

use candid::Principal;
pub use types::*;
pub use storage::*;
pub use errors::*;

// Re-export main functions
pub use policy_manager::{create_policy, subscribe};

// Initialization
#[ic_cdk::init]
fn init(treasury: Principal) {
    storage::init_state(treasury);
}

// Administrative functions
#[ic_cdk::update]
fn set_payout_engine(payout_engine: Principal) -> Result<()> {
    policy_manager::set_payout_engine(payout_engine)
}

// Query functions
#[ic_cdk::query]
fn get_policy_details(policy_id: u64) -> Result<PolicyDetails> {
    policy_manager::get_policy_details(policy_id)
}

#[ic_cdk::query]
fn get_historical_subscribers(policy_id: u64, season: u64) -> Vec<Principal> {
    policy_manager::get_historical_subscribers(policy_id, season)
}

#[ic_cdk::query]
fn get_platform_stats() -> (u64, u64, u64) {
    storage::get_platform_stats()
}

#[ic_cdk::query]
fn get_state() -> State {
    storage::get_state()
}

// Export the candid interface
ic_cdk::export_candid!();