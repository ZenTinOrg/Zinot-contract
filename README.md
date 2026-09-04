# Zinot Contract - Stellar-Native Lending Protocol

Comprehensive Soroban smart contracts for Zinot, a fully integrated lending and liquidity protocol on Stellar.

## 🌟 Overview

Zinot Protocol leverages **Stellar's fast finality, low fees, and native asset model** combined with **Soroban smart contracts** to deliver a transparent, efficient lending experience.

### Core Features

✅ **Supply & Borrow Operations**
- Supply USDC and XLM to earn interest
- Borrow against collateral with transparent rates
- Interest accrual based on pool utilization

✅ **Risk Management**
- Health factor enforcement (minimum 1.25)
- Collateral factor validation per asset
- Real-time liquidation mechanism
- Emergency circuit breaker

✅ **Advanced Features**
- Flash loans (0.05% fee)
- Liquidation auctions with bonuses
- Dynamic interest rate curves
- Multi-collateral support
- Governance & proposal voting
- Timelock on critical operations

✅ **Asset Support**
- **USDC** (collateral factor: 80%)
- **XLM** (collateral factor: 75%)
- Extensible asset registry

## 🏗️ Architecture

### Stellar Integration

```
┌─────────────────────────────────┐
│   Stellar Blockchain (soroban)  │
│  ┌──────────────────────────────┐│
│  │  Zinot Pool Contract         ││
│  │  - Supply/Borrow/Repay       ││
│  │  - Liquidations              ││
│  │  - Flash Loans               ││
│  │  - Governance                ││
│  └──────────────────────────────┘│
│  ┌──────────────────────────────┐│
│  │  USDC & XLM Token Contracts  ││
│  └──────────────────────────────┘│
└─────────────────────────────────┘
         ↕ Events & State
┌─────────────────────────────────┐
│  Off-Chain Indexer (Backend)    │
│  - Event parsing                │
│  - State aggregation            │
│  - Market statistics            │
└─────────────────────────────────┘
         ↕ APIs
┌─────────────────────────────────┐
│  Frontend (React)               │
│  - Market data display          │
│  - User interactions            │
│  - Wallet integration           │
└─────────────────────────────────┘
```

### Module Structure

- **core**: Supply, borrow, repay, withdraw operations
- **storage**: Persistent state management with Soroban storage
- **risk**: Health factor, liquidation, and risk calculations
- **liquidation**: Liquidation mechanisms and auctions
- **interest**: Dynamic interest accrual and APY calculations
- **governance**: Proposal voting and parameter updates
- **oracle**: Price feed integration for collateral valuation
- **flashloan**: Atomic loan functionality
- **admin**: Administrative controls and upgrades
- **monitoring**: Event emission and metrics

## 🚀 Stellar Features Utilized

### Native Assets
- Direct interaction with Stellar's native XLM
- Support for custom issued assets (USDC via Circle)
- No wrapped tokens required

### Fast Settlement
- Stellar's ~5-second block time
- Immediate finality
- Ideal for DeFi operations

### Low Fees
- Minimal transaction costs
- Economical collateral management
- Affordable liquidations

### Soroban Smart Contracts
- WASM-based deterministic execution
- Direct asset control via trustlines
- Efficient storage model
- Built-in authorization framework

## 📋 Interest Rate Model

**Utilization-Based Variable Rates:**

```
0% - 80% utilization:
  Borrow Rate = 2% + (4% × Utilization / 100)

80% - 100% utilization:
  Borrow Rate = 5.2% + (50% × (Utilization - 80%) / 20%)

Supply APY = Borrow Rate × Utilization × (1 - Reserve Factor)
```

## 🔒 Safety Mechanisms

### Risk Parameters
- **Health Factor Floor**: 1.25 (125 basis points)
- **Collateral Factors**: 30-95% per asset
- **Liquidation Bonus**: 10% incentive
- **Isolation Mode**: Debt ceiling per asset

### Emergency Controls
- Circuit breaker for catastrophic events
- Admin pause functionality
- Emergency withdrawal system
- Timelock on upgrades (2-day delay)

### Validation
- Input amount validation
- Price feed freshness checks
- Overflow/underflow protection
- Rate parameter bounds

## 🔗 Contract Interaction Flow

### Supply Flow
```
User → Approve Token → Supply → Earn APY
                           ↓
                    Contract holds token
                    Updates balance in storage
                    Increases pool liquidity
```

### Borrow Flow
```
User → Check Collateral → Borrow → Repay Over Time
             ↓
       Health Factor ≥ 1.25?
             ↓
       Transfer asset to user
       Record debt in storage
       Update utilization rates
```

### Liquidation Flow
```
Monitor Health Factor
       ↓
   HF < 1.0?
       ↓
Liquidator Repays Debt → Seizes Collateral → Gets Bonus
       ↓
Borrower's position improves
Pool's reserves increase
```

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Soroban CLI
- Stellar testnet account

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Deploying to Testnet
```bash
soroban contract deploy \
  --wasm-ref target/wasm32-unknown-unknown/release/zinot_pool.wasm
```

## 📊 Pool Statistics

The contract tracks:
- Total liquidity per asset
- Total borrowed per asset
- Utilization rates
- APY curves
- Liquidation events
- Reserve factor accumulation

## 🔄 State Management

**Persistent Storage:**
- User balances (supplied & borrowed)
- Pool statistics
- Collateral factors
- Interest rates
- Admin address
- Fee accumulators

**State Keys:**
```rust
Balance(User, Asset) → Amount
Debt(User, Asset) → Amount
TotalLiquidity(Asset) → Amount
TotalBorrowed(Asset) → Amount
CollateralFactor(Asset) → Factor
InterestRate(Asset) → Rate
```

## 🎯 Integration with Zinot Backend

The contract emits events that the backend indexes:
- `SupplyEvent`: Tracked for market analytics
- `BorrowEvent`: Used for utilization calculations
- `LiquidationEvent`: Monitored for risk metrics
- `WithdrawEvent`: Updates liquidity pools

Backend aggregates these events and exposes via REST API.

## 🌐 Frontend Integration

The frontend consumes contract state via:
1. **Direct queries** to contract methods (view functions)
2. **Backend API** for historical data
3. **Event streaming** for real-time updates
4. **Price feeds** for collateral valuation

## 🤝 Contributing

We welcome contributions for:
- Additional risk parameters
- Optimized interest models
- Additional collateral assets
- Governance enhancements
- Oracle improvements
- Security audits

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is open source and available under the MIT License.

## 👤 Maintainer

**m1s0g1** - Zinot Protocol Maintainer  
Email: danielegbezien@gmail.com

---

**Built on Stellar. Powered by Soroban. For everyone.**
