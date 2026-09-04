//! Read-only view functions for querying pool state

use soroban_sdk::{Address, Env};
use crate::storage::Storage;
use crate::risk::RiskCalc;
use crate::interest::InterestAccrual;

pub struct ViewFunctions;

impl ViewFunctions {
    /// Get user's position summary
    pub fn get_position(
        env: &Env,
        user: &Address,
        collateral_asset: &Address,
        debt_asset: &Address,
    ) -> (i128, i128, i128) {
        let collateral = Storage::get_balance(env, user, collateral_asset);
        let debt = Storage::get_debt(env, user, debt_asset);
        let health_factor = RiskCalc::calculate_health_factor(env, user, collateral_asset, debt_asset);

        (collateral, debt, health_factor)
    }

    /// Get current borrow APY for asset
    pub fn get_borrow_apy(env: &Env, asset: &Address) -> u32 {
        InterestAccrual::calculate_apy(env, asset)
    }

    /// Get current supply APY for asset
    pub fn get_supply_apy(env: &Env, asset: &Address) -> u32 {
        InterestAccrual::calculate_supply_apy(env, asset)
    }

    /// Check if user can borrow additional amount
    pub fn can_borrow(
        env: &Env,
        user: &Address,
        collateral_asset: &Address,
        debt_asset: &Address,
        amount: i128,
    ) -> bool {
        let collateral = Storage::get_balance(env, user, collateral_asset);
        let collateral_factor = Storage::get_collateral_factor(env, collateral_asset);
        let max_borrow = (collateral * collateral_factor as i128) / 100;

        let current_debt = Storage::get_debt(env, user, debt_asset);
        current_debt + amount <= max_borrow
    }

    /// Get max borrowable amount
    pub fn get_max_borrow(
        env: &Env,
        user: &Address,
        collateral_asset: &Address,
    ) -> i128 {
        let collateral = Storage::get_balance(env, user, collateral_asset);
        let collateral_factor = Storage::get_collateral_factor(env, collateral_asset);
        (collateral * collateral_factor as i128) / 100
    }
}
