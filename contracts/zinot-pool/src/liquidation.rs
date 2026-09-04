//! Liquidation logic for Zinot Pool

use soroban_sdk::{Address, Env};
use crate::storage::Storage;
use crate::risk::RiskCalc;

pub struct Liquidation;

impl Liquidation {
    /// Liquidate a user's collateral when health factor drops below 1.0
    pub fn liquidate(
        env: &Env,
        liquidator: &Address,
        borrower: &Address,
        collateral_asset: &Address,
        debt_asset: &Address,
        repay_amount: i128,
    ) {
        // Check if borrower is liquidatable
        let health_factor = RiskCalc::calculate_health_factor(
            env,
            borrower,
            collateral_asset,
            debt_asset,
        );

        if health_factor >= 100 {
            panic!("Health factor is healthy, cannot liquidate");
        }

        let borrower_debt = Storage::get_debt(env, borrower, debt_asset);
        if repay_amount > borrower_debt {
            panic!("Repay amount exceeds total debt");
        }

        // Liquidator repays the debt
        let token = soroban_sdk::token::Client::new(env, debt_asset);
        token.transfer(liquidator, &env.current_contract_address(), &repay_amount);

        // Update borrower debt
        Storage::set_debt(env, borrower, debt_asset, borrower_debt - repay_amount);

        // Calculate liquidation bonus (e.g., 10%)
        let liquidation_bonus = (repay_amount * 10) / 100;
        let collateral_to_seize = repay_amount + liquidation_bonus;

        // Seize collateral and transfer to liquidator
        let borrower_collateral = Storage::get_balance(env, borrower, collateral_asset);
        let seizeable = std::cmp::min(collateral_to_seize, borrower_collateral);

        Storage::set_balance(
            env,
            borrower,
            collateral_asset,
            borrower_collateral - seizeable,
        );

        let collateral_token = soroban_sdk::token::Client::new(env, collateral_asset);
        collateral_token.transfer(
            &env.current_contract_address(),
            liquidator,
            &seizeable,
        );
    }

    /// Check if a position can be liquidated
    pub fn is_liquidatable(env: &Env, borrower: &Address, collateral_asset: &Address, debt_asset: &Address) -> bool {
        let health_factor = RiskCalc::calculate_health_factor(
            env,
            borrower,
            collateral_asset,
            debt_asset,
        );
        health_factor < 100
    }
}
