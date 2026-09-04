//! Storage logic for Zinot Pool
//!
//! This module handles the persistent state of the contract.
//! We use Soroban's storage types (Persistent, Instance, Temporary)
//! to manage user balances and global pool data.

use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address, Address),       // (User, Asset) -> Balance
    Debt(Address, Address),          // (User, Asset) -> Debt
    TotalLiquidity(Address),         // Asset -> Total Amount
    TotalBorrowed(Address),          // Asset -> Total Borrowed
    CollateralFactor(Address),       // Asset -> Collateral Factor (e.g., 75 = 75%)
    InterestRate(Address),           // Asset -> Interest Rate (percentage)
    PoolAdmin,                        // Admin address
    AssetList,                        // List of supported assets
}

#[contracttype]
#[derive(Clone)]
pub struct PoolStats {
    pub total_liquidity: i128,
    pub total_borrowed: i128,
    pub collateral_factor: u32,
    pub interest_rate: u32,
}

pub struct Storage;

impl Storage {
    pub fn get_balance(env: &Env, user: &Address, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get::<DataKey, i128>(&DataKey::Balance(user.clone(), asset.clone()))
            .unwrap_or(0)
    }

    pub fn set_balance(env: &Env, user: &Address, asset: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone(), asset.clone()), &amount);
    }

    pub fn get_debt(env: &Env, user: &Address, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get::<DataKey, i128>(&DataKey::Debt(user.clone(), asset.clone()))
            .unwrap_or(0)
    }

    pub fn set_debt(env: &Env, user: &Address, asset: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::Debt(user.clone(), asset.clone()), &amount);
    }

    pub fn get_total_liquidity(env: &Env, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get::<DataKey, i128>(&DataKey::TotalLiquidity(asset.clone()))
            .unwrap_or(0)
    }

    pub fn set_total_liquidity(env: &Env, asset: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::TotalLiquidity(asset.clone()), &amount);
    }

    pub fn get_total_borrowed(env: &Env, asset: &Address) -> i128 {
        env.storage()
            .persistent()
            .get::<DataKey, i128>(&DataKey::TotalBorrowed(asset.clone()))
            .unwrap_or(0)
    }

    pub fn set_total_borrowed(env: &Env, asset: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::TotalBorrowed(asset.clone()), &amount);
    }

    pub fn get_collateral_factor(env: &Env, asset: &Address) -> u32 {
        env.storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::CollateralFactor(asset.clone()))
            .unwrap_or(75) // Default 75%
    }

    pub fn set_collateral_factor(env: &Env, asset: &Address, factor: u32) {
        env.storage()
            .persistent()
            .set(&DataKey::CollateralFactor(asset.clone()), &factor);
    }

    pub fn get_interest_rate(env: &Env, asset: &Address) -> u32 {
        env.storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::InterestRate(asset.clone()))
            .unwrap_or(5) // Default 5%
    }

    pub fn set_interest_rate(env: &Env, asset: &Address, rate: u32) {
        env.storage()
            .persistent()
            .set(&DataKey::InterestRate(asset.clone()), &rate);
    }

    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage()
            .persistent()
            .get::<DataKey, Address>(&DataKey::PoolAdmin)
            .ok()
    }

    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage()
            .persistent()
            .set(&DataKey::PoolAdmin, admin);
    }
}
