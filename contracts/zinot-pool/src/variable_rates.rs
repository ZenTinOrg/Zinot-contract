//! Variable interest rate models

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct VariableRateModel;

impl VariableRateModel {
    /// Calculate current borrow rate based on utilization
    pub fn get_borrow_rate(env: &Env, asset: &Address) -> u32 {
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 2; // Min 2%
        }

        let utilization = (total_borrowed * 100) / total_liquidity;

        // Piecewise linear model
        // 0-80%: 2% + 4% * utilization
        // 80-100%: 5.2% + 50% * (utilization - 80%)
        if utilization <= 80 {
            2 + ((4 * utilization as u32) / 100)
        } else {
            let excess = utilization - 80;
            52 + ((50 * excess as u32) / 20)
        }
    }

    /// Calculate supply rate (borrow rate * utilization * (1 - reserve factor))
    pub fn get_supply_rate(env: &Env, asset: &Address, reserve_factor: u32) -> u32 {
        let borrow_rate = Self::get_borrow_rate(env, asset);
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 0;
        }

        let utilization = (total_borrowed * 100) / total_liquidity as u32;
        let available_for_suppliers = 100 - reserve_factor;
        (borrow_rate * utilization * available_for_suppliers) / 10000
    }
}
