//! Allowance and permit management for contract interactions

use soroban_sdk::{Address, Env};

pub struct AllowanceManager;

impl AllowanceManager {
    /// Grant allowance for a spender
    pub fn approve(env: &Env, owner: &Address, spender: &Address, amount: i128) {
        owner.require_auth();
        // TODO: Store allowance for owner -> spender
    }

    /// Get remaining allowance
    pub fn get_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
        // TODO: Query allowance
        0
    }

    /// Execute with permit (offline approval)
    pub fn permit(env: &Env, owner: &Address, spender: &Address, amount: i128, signature: &[u8]) {
        // TODO: Verify EIP-712 signature, grant allowance
    }
}
