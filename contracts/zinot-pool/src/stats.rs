//! Pool statistics and analytics

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

#[derive(Clone, Debug)]
pub struct PoolStats {
    pub total_liquidity: i128,
    pub total_borrowed: i128,
    pub utilization_rate: u32,
    pub average_interest_rate: u32,
}

pub struct StatsCalculator;

impl StatsCalculator {
    /// Get comprehensive pool statistics
    pub fn get_pool_stats(env: &Env, asset: &Address) -> PoolStats {
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        let utilization_rate = if total_liquidity == 0 {
            0
        } else {
            ((total_borrowed * 100) / total_liquidity) as u32
        };

        let average_interest_rate = Storage::get_interest_rate(env, asset);

        PoolStats {
            total_liquidity,
            total_borrowed,
            utilization_rate,
            average_interest_rate,
        }
    }

    /// Get available liquidity for borrowing
    pub fn get_available_liquidity(env: &Env, asset: &Address) -> i128 {
        let total = Storage::get_total_liquidity(env, asset);
        let borrowed = Storage::get_total_borrowed(env, asset);
        total - borrowed
    }

    /// Get utilization rate as percentage
    pub fn get_utilization_rate(env: &Env, asset: &Address) -> u32 {
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 0;
        }

        ((total_borrowed * 100) / total_liquidity) as u32
    }
}
