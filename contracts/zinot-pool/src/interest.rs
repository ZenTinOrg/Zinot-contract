//! Interest accrual and APY calculations

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct InterestAccrual;

impl InterestAccrual {
    /// Calculate accrued interest for a debt position
    pub fn calculate_interest(
        env: &Env,
        principal: i128,
        blocks_elapsed: u32,
        asset: &Address,
    ) -> i128 {
        let interest_rate = Storage::get_interest_rate(env, asset);
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 0;
        }

        // Utilization rate
        let utilization = (total_borrowed * 100) / total_liquidity;

        // Interest compounds based on utilization
        // Formula: interest = principal * rate * utilization * blocks / blocks_per_year
        let adjusted_rate = (interest_rate as i128 * utilization) / 100;
        (principal * adjusted_rate * blocks_elapsed as i128) / (365 * 24 * 3600)
    }

    /// Calculate APY (Annual Percentage Yield)
    pub fn calculate_apy(
        env: &Env,
        asset: &Address,
    ) -> u32 {
        let interest_rate = Storage::get_interest_rate(env, asset);
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return interest_rate;
        }

        let utilization = ((total_borrowed * 100) / total_liquidity) as u32;
        interest_rate + ((utilization * 2) / 100)
    }

    /// Calculate supply APY (interest earned by suppliers)
    pub fn calculate_supply_apy(
        env: &Env,
        asset: &Address,
    ) -> u32 {
        let borrow_apy = Self::calculate_apy(env, asset);
        let total_liquidity = Storage::get_total_liquidity(env, asset);
        let total_borrowed = Storage::get_total_borrowed(env, asset);

        if total_liquidity == 0 {
            return 0;
        }

        let utilization = ((total_borrowed * 100) / total_liquidity) as u32;
        (borrow_apy * utilization) / 100
    }
}
