//! Storage logic for Zinot Pool
//!
//! This module handles the persistent state of the contract.
//! We use Soroban's storage types (Persistent, Instance, Temporary)
//! to manage user balances and global pool data.

use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address, Address), // (User, Asset) -> Balance
    Debt(Address, Address),    // (User, Asset) -> Debt
    TotalLiquidity(Address),   // Asset -> Total Amount
}

// Storage helpers would go here
