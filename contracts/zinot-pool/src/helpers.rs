//! Helper functions for common operations

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct PoolHelpers;

impl PoolHelpers {
    /// Calculate collateral value in terms of another asset
    pub fn collateral_value(
        env: &Env,
        collateral_amount: i128,
        collateral_asset: &Address,
    ) -> i128 {
        let collateral_factor = Storage::get_collateral_factor(env, collateral_asset);
        (collateral_amount * collateral_factor as i128) / 100
    }

    /// Check if amount exceeds available liquidity
    pub fn exceeds_liquidity(
        env: &Env,
        amount: i128,
        asset: &Address,
    ) -> bool {
        let available = Storage::get_total_liquidity(env, asset) - Storage::get_total_borrowed(env, asset);
        amount > available
    }

    /// Normalize amount to asset decimals
    pub fn normalize_amount(amount: i128, from_decimals: u32, to_decimals: u32) -> i128 {
        if from_decimals == to_decimals {
            return amount;
        }
        if from_decimals > to_decimals {
            amount / (10_i128.pow(from_decimals - to_decimals))
        } else {
            amount * (10_i128.pow(to_decimals - from_decimals))
        }
    }
}
