//! Pool metadata and configuration

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct PoolMetadata {
    pub admin: Address,
    pub supported_assets: u32,
    pub total_collateral_value: i128,
    pub total_debt_value: i128,
    pub creation_block: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct AssetConfig {
    pub asset: Address,
    pub collateral_factor: u32,
    pub interest_rate: u32,
    pub is_collateral: bool,
    pub decimals: u32,
}

pub fn create_asset_config(
    asset: Address,
    collateral_factor: u32,
    interest_rate: u32,
    is_collateral: bool,
    decimals: u32,
) -> AssetConfig {
    AssetConfig {
        asset,
        collateral_factor,
        interest_rate,
        is_collateral,
        decimals,
    }
}
