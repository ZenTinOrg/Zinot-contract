//! Event logging for Zinot Pool

use soroban_sdk::{contracttype, Address};

#[contracttype]
pub enum ZinotEvent {
    SupplyEvent {
        supplier: Address,
        asset: Address,
        amount: i128,
    },
    BorrowEvent {
        borrower: Address,
        asset: Address,
        amount: i128,
    },
    RepayEvent {
        repayer: Address,
        asset: Address,
        amount: i128,
    },
    WithdrawEvent {
        withdrawer: Address,
        asset: Address,
        amount: i128,
    },
    LiquidationEvent {
        liquidator: Address,
        borrower: Address,
        collateral_asset: Address,
        debt_asset: Address,
        repay_amount: i128,
    },
}
