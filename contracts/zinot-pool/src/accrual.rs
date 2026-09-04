//! Interest accrual scheduling and updates

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct AccrualEngine;

impl AccrualEngine {
    /// Update interest accrual for an asset's borrowers
    pub fn accrue_interest(env: &Env, asset: &Address, blocks_elapsed: u32) {
        // TODO: Calculate accrued interest
        // TODO: Update cumulative interest index
    }

    /// Update accrual index for asset
    pub fn update_accrual_index(env: &Env, asset: &Address) {
        // TODO: Store new accrual index
    }

    /// Get current accrual index
    pub fn get_accrual_index(env: &Env, asset: &Address) -> i128 {
        // TODO: Retrieve accrual index
        1_000_000_000_000_000_000 // 1e18
    }
}
