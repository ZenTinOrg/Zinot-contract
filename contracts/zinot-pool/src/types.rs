//! Common types and constants

use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug)]
pub struct Position {
    pub owner: soroban_sdk::Address,
    pub collateral_asset: soroban_sdk::Address,
    pub collateral_amount: i128,
    pub debt_asset: soroban_sdk::Address,
    pub debt_amount: i128,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum OperationType {
    Supply,
    Borrow,
    Repay,
    Withdraw,
    Liquidate,
}

pub const MIN_HEALTH_FACTOR: i128 = 100; // 1.0 scaled by 100
pub const MAX_COLLATERAL_FACTOR: u32 = 95; // 95% max
pub const MIN_COLLATERAL_FACTOR: u32 = 30; // 30% min
pub const DEFAULT_INTEREST_RATE: u32 = 5; // 5% default
pub const MAX_INTEREST_RATE: u32 = 100; // 100% max
