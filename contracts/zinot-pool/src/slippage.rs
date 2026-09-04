//! Slippage protection and swap limits

use soroban_sdk::{Address, Env};

pub struct SlippageProtection;

impl SlippageProtection {
    /// Check if swap slips within tolerance
    pub fn check_slippage(
        env: &Env,
        amount_in: i128,
        amount_out_min: i128,
        actual_amount_out: i128,
    ) -> bool {
        actual_amount_out >= amount_out_min
    }

    /// Calculate slippage percentage
    pub fn calculate_slippage_percent(amount_out_expected: i128, amount_out_actual: i128) -> u32 {
        if amount_out_expected == 0 {
            return 0;
        }
        ((amount_out_expected - amount_out_actual) * 100) / amount_out_expected as u32
    }

    /// Get safe slippage tolerance (3%)
    pub fn get_safe_slippage_tolerance() -> u32 {
        3
    }
}
