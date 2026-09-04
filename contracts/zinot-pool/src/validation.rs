//! Input validation for pool operations

pub struct Validator;

impl Validator {
    /// Validate amount is positive
    pub fn validate_amount(amount: i128) -> Result<(), &'static str> {
        if amount <= 0 {
            Err("Amount must be positive")
        } else {
            Ok(())
        }
    }

    /// Validate collateral factor
    pub fn validate_collateral_factor(factor: u32) -> Result<(), &'static str> {
        if factor > 100 {
            Err("Collateral factor cannot exceed 100%")
        } else if factor == 0 {
            Err("Collateral factor must be positive")
        } else {
            Ok(())
        }
    }

    /// Validate interest rate
    pub fn validate_interest_rate(rate: u32) -> Result<(), &'static str> {
        if rate > 100 {
            Err("Interest rate cannot exceed 100%")
        } else {
            Ok(())
        }
    }

    /// Validate health factor
    pub fn validate_health_factor(health_factor: i128) -> Result<(), &'static str> {
        if health_factor < 100 {
            Err("Health factor below minimum (1.0)")
        } else {
            Ok(())
        }
    }
}
