//! Admin functions for pool management

use soroban_sdk::{Address, Env};
use crate::storage::Storage;

pub struct AdminFunctions;

impl AdminFunctions {
    /// Update collateral factor for an asset
    pub fn update_collateral_factor(env: &Env, admin: &Address, asset: &Address, factor: u32) {
        Self::verify_admin(env, admin);
        if factor > 100 {
            panic!("Collateral factor cannot exceed 100%");
        }
        Storage::set_collateral_factor(env, asset, factor);
    }

    /// Update interest rate for an asset
    pub fn update_interest_rate(env: &Env, admin: &Address, asset: &Address, rate: u32) {
        Self::verify_admin(env, admin);
        if rate > 100 {
            panic!("Interest rate cannot exceed 100%");
        }
        Storage::set_interest_rate(env, asset, rate);
    }

    /// Transfer admin privileges
    pub fn transfer_admin(env: &Env, current_admin: &Address, new_admin: &Address) {
        Self::verify_admin(env, current_admin);
        Storage::set_admin(env, new_admin);
    }

    /// Verify that caller is admin
    fn verify_admin(env: &Env, caller: &Address) {
        let admin = Storage::get_admin(env);
        match admin {
            Some(admin_addr) => {
                if admin_addr != *caller {
                    panic!("Unauthorized: admin only");
                }
            }
            None => panic!("No admin set"),
        }
    }
}
