//! Protocol constants and configuration

pub mod pool {
    pub const LIQUIDATION_BONUS: u32 = 10; // 10% bonus for liquidators
    pub const CLOSE_FACTOR: u32 = 50; // Max 50% of debt can be repaid in liquidation
    pub const MIN_BORROW_AMOUNT: i128 = 1_000_000; // Dust limit
    pub const MIN_SUPPLY_AMOUNT: i128 = 1_000_000;
}

pub mod rates {
    pub const BASE_RATE: u32 = 2; // 2% base interest rate
    pub const SLOPE_1: u32 = 4; // 4% rate per 100% utilization (0-80%)
    pub const SLOPE_2: u32 = 50; // 50% rate per 100% utilization (80-100%)
    pub const KINK_UTILIZATION: u32 = 80; // 80% utilization kink
}

pub mod collateral {
    pub const USDC_COLLATERAL_FACTOR: u32 = 80;
    pub const XLM_COLLATERAL_FACTOR: u32 = 75;
    pub const DEFAULT_COLLATERAL_FACTOR: u32 = 70;
}

pub mod risk {
    pub const MIN_HEALTH_FACTOR: i128 = 125; // 1.25 scaled by 100
    pub const LIQUIDATION_THRESHOLD: i128 = 100; // 1.0
}

pub mod time {
    pub const SECONDS_PER_YEAR: u64 = 365 * 24 * 3600;
    pub const BLOCKS_PER_YEAR: u32 = 365 * 24 * 3600; // Assuming 1 block per second
}
