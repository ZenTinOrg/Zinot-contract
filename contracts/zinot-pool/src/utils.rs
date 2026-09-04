//! Utility functions for Zinot Pool

pub struct MathUtils;

impl MathUtils {
    /// Safe multiply with overflow check
    pub fn safe_mul(a: i128, b: i128) -> Option<i128> {
        a.checked_mul(b)
    }

    /// Safe divide with zero check
    pub fn safe_div(a: i128, b: i128) -> Option<i128> {
        if b == 0 {
            return None;
        }
        Some(a / b)
    }

    /// Calculate percentage of amount
    pub fn percentage(amount: i128, percent: u32) -> i128 {
        (amount * percent as i128) / 100
    }

    /// Calculate percentage with scaling (e.g., for basis points)
    pub fn percentage_scaled(amount: i128, percent: u32, scale: u32) -> i128 {
        (amount * percent as i128) / scale as i128
    }

    /// Min of two values
    pub fn min(a: i128, b: i128) -> i128 {
        if a < b { a } else { b }
    }

    /// Max of two values
    pub fn max(a: i128, b: i128) -> i128 {
        if a > b { a } else { b }
    }
}

/// Precision constants
pub mod precision {
    pub const USDC_DECIMALS: u32 = 6;
    pub const XLM_DECIMALS: u32 = 7;
    pub const PRECISION_MULTIPLIER: i128 = 1_000_000_000_000_000_000; // 1e18
}
