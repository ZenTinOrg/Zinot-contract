//! Error types for Zinot Pool contract

#[derive(Clone, Debug)]
pub enum ZinotError {
    PoolAlreadyInitialized,
    InvalidAmount,
    InsufficientLiquidity,
    InsufficientCollateral,
    InsufficientBalance,
    RepayAmountExceedsDebt,
    HealthFactorTooLow,
    AssetNotSupported,
    UnauthorizedAdmin,
}

impl ZinotError {
    pub fn panic(&self) -> ! {
        match self {
            ZinotError::PoolAlreadyInitialized => panic!("Pool already initialized"),
            ZinotError::InvalidAmount => panic!("Invalid amount: must be positive"),
            ZinotError::InsufficientLiquidity => panic!("Insufficient pool liquidity"),
            ZinotError::InsufficientCollateral => panic!("Insufficient collateral for borrow"),
            ZinotError::InsufficientBalance => panic!("Insufficient balance"),
            ZinotError::RepayAmountExceedsDebt => panic!("Repay amount exceeds debt"),
            ZinotError::HealthFactorTooLow => panic!("Health factor below minimum"),
            ZinotError::AssetNotSupported => panic!("Asset not supported"),
            ZinotError::UnauthorizedAdmin => panic!("Unauthorized: admin only"),
        }
    }
}
