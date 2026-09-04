//! Risk calculations for Zinot Pool

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct RiskCalc;

impl RiskCalc {
    /// Calculate health factor for a user
    /// Health Factor = (Collateral Value * Collateral Factor) / Debt Value
    /// Formula: HF > 1.0 is healthy
    pub fn calculate_health_factor(
        env: &Env,
        user: &Address,
        collateral_asset: &Address,
        borrow_asset: &Address,
    ) -> i128 {
        let collateral = Storage::get_balance(env, user, collateral_asset);
        let debt = Storage::get_debt(env, user, borrow_asset);

        if debt == 0 {
            return i128::MAX; // Infinite health if no debt
        }

        let collateral_factor = Storage::get_collateral_factor(env, collateral_asset);
        let collateral_value = (collateral * collateral_factor as i128) / 100;

        if collateral_value == 0 {
            return 0;
        }

        (collateral_value * 100) / debt
    }

    /// Check if user is in liquidation zone (HF < 1.5)
    pub fn is_liquidation_risk(health_factor: i128) -> bool {
        health_factor < 150 // 150 represents 1.5 when scaled by 100
    }

    /// Calculate utilization rate of asset pool
    /// Utilization = Total Borrowed / Total Liquidity
    pub fn calculate_utilization(env: &Env, asset: &Address) -> u32 {
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 0;
        }

        ((total_borrowed * 100) / total_liquidity) as u32
    }

    /// Calculate interest accrued based on utilization
    pub fn calculate_accrued_interest(
        env: &Env,
        principal: i128,
        blocks_elapsed: u32,
        asset: &Address,
    ) -> i128 {
        let interest_rate = Storage::get_interest_rate(env, asset);
        let utilization = Self::calculate_utilization(env, asset);

        // Interest = Principal * (BaseRate + Utilization * Multiplier) * BlocksElapsed / BlocksPerYear
        let adjusted_rate = interest_rate + ((utilization as u32 * 2) / 100);
        (principal * adjusted_rate as i128 * blocks_elapsed as i128) / (365 * 24 * 3600)
    }
}
