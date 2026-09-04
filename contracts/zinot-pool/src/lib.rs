//! Zinot Pool Contract
//!
//! This is the core contract for the Zinot liquidity protocol.
//! It manages deposits, borrows, and collateral for USDC and XLM.

#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

mod storage;
mod error;
mod risk;
mod events;
mod liquidation;
mod metadata;
mod utils;
mod admin;
mod interest;
mod validation;
mod stats;
mod types;
mod constants;

use storage::Storage;

#[contract]
pub struct ZinotPool;

#[contractimpl]
impl ZinotPool {
    /// Initialize the pool with an admin address
    pub fn init(env: Env, admin: Address) {
        if Storage::get_admin(&env).is_some() {
            panic!("Pool already initialized");
        }
        Storage::set_admin(&env, &admin);
    }

    /// Supply assets to the pool to earn interest.
    pub fn supply(env: Env, supplier: Address, asset: Address, amount: i128) {
        supplier.require_auth();

        if amount <= 0 {
            panic!("Supply amount must be positive");
        }

        // Transfer tokens from supplier to contract
        let token = token::Client::new(&env, &asset);
        token.transfer(&supplier, &env.current_contract_address(), &amount);

        // Update supplier balance
        let current_balance = Storage::get_balance(&env, &supplier, &asset);
        Storage::set_balance(&env, &supplier, &asset, current_balance + amount);

        // Update total liquidity
        let total = Storage::get_total_liquidity(&env, &asset);
        Storage::set_total_liquidity(&env, &asset, total + amount);
    }

    /// Borrow assets from the pool using supplied assets as collateral.
    pub fn borrow(env: Env, borrower: Address, borrow_asset: Address, borrow_amount: i128) {
        borrower.require_auth();

        if borrow_amount <= 0 {
            panic!("Borrow amount must be positive");
        }

        // Check available liquidity
        let available = Storage::get_total_liquidity(&env, &borrow_asset);
        if borrow_amount > available {
            panic!("Insufficient pool liquidity");
        }

        // Check collateral (for simplicity, assume USDC and XLM, with USDC as collateral)
        let collateral_balance = Storage::get_balance(&env, &borrower, &borrow_asset);
        let collateral_factor = Storage::get_collateral_factor(&env, &borrow_asset);
        let max_borrow = (collateral_balance * collateral_factor as i128) / 100;

        let current_debt = Storage::get_debt(&env, &borrower, &borrow_asset);
        if current_debt + borrow_amount > max_borrow {
            panic!("Insufficient collateral for borrow");
        }

        // Update debt
        Storage::set_debt(&env, &borrower, &borrow_asset, current_debt + borrow_amount);

        // Update total borrowed
        let total_borrowed = Storage::get_total_borrowed(&env, &borrow_asset);
        Storage::set_total_borrowed(&env, &borrow_asset, total_borrowed + borrow_amount);

        // Transfer to borrower
        let token = token::Client::new(&env, &borrow_asset);
        token.transfer(&env.current_contract_address(), &borrower, &borrow_amount);
    }

    /// Repay borrowed assets.
    pub fn repay(env: Env, borrower: Address, asset: Address, repay_amount: i128) {
        borrower.require_auth();

        if repay_amount <= 0 {
            panic!("Repay amount must be positive");
        }

        let current_debt = Storage::get_debt(&env, &borrower, &asset);
        if repay_amount > current_debt {
            panic!("Repay amount exceeds debt");
        }

        // Transfer tokens from borrower to contract
        let token = token::Client::new(&env, &asset);
        token.transfer(&borrower, &env.current_contract_address(), &repay_amount);

        // Update debt
        Storage::set_debt(&env, &borrower, &asset, current_debt - repay_amount);

        // Update total borrowed
        let total_borrowed = Storage::get_total_borrowed(&env, &asset);
        Storage::set_total_borrowed(&env, &asset, total_borrowed - repay_amount);
    }

    /// Withdraw supplied assets.
    pub fn withdraw(env: Env, supplier: Address, asset: Address, withdraw_amount: i128) {
        supplier.require_auth();

        if withdraw_amount <= 0 {
            panic!("Withdraw amount must be positive");
        }

        let balance = Storage::get_balance(&env, &supplier, &asset);
        if withdraw_amount > balance {
            panic!("Insufficient balance to withdraw");
        }

        // Update balance
        Storage::set_balance(&env, &supplier, &asset, balance - withdraw_amount);

        // Update total liquidity
        let total = Storage::get_total_liquidity(&env, &asset);
        Storage::set_total_liquidity(&env, &asset, total - withdraw_amount);

        // Transfer to supplier
        let token = token::Client::new(&env, &asset);
        token.transfer(&env.current_contract_address(), &supplier, &withdraw_amount);
    }

    /// Get user balance for an asset
    pub fn get_balance(env: Env, user: Address, asset: Address) -> i128 {
        Storage::get_balance(&env, &user, &asset)
    }

    /// Get user debt for an asset
    pub fn get_debt(env: Env, user: Address, asset: Address) -> i128 {
        Storage::get_debt(&env, &user, &asset)
    }

    /// Get total liquidity in pool for an asset
    pub fn get_total_liquidity(env: Env, asset: Address) -> i128 {
        Storage::get_total_liquidity(&env, &asset)
    }

    /// Get total borrowed from pool for an asset
    pub fn get_total_borrowed(env: Env, asset: Address) -> i128 {
        Storage::get_total_borrowed(&env, &asset)
    }
}

#[cfg(test)]
mod test;
