# Zinot Pool Contract

Comprehensive Soroban smart contract implementation for the Zinot lending protocol.

## Modules

- **core**: Supply, borrow, repay, withdraw operations
- **storage**: Persistent state management
- **risk**: Health factor and risk calculations
- **liquidation**: Liquidation mechanisms
- **admin**: Administrative functions
- **interest**: Interest accrual and APY calculations
- **oracle**: Price feeds and asset valuations
- **governance**: Protocol governance
- **rewards**: Incentive programs
- **flashloan**: Flash loan functionality
- **auctions**: Liquidation auctions
- **stablecoin**: Synthetic stablecoin issuance

## Safety Features

- Health factor enforcement (minimum 1.25)
- Collateral factor validation
- Isolation mode for new assets
- Price feed freshness checks
- Emergency pause functionality
- Timelock on upgrades

## Getting Started

See parent README for build and test instructions.
