# Zinot Contract

Soroban smart contracts for Zinot, a Stellar-native liquidity and lending protocol.

## What this project is

This repo contains the on-chain core of Zinot. The contract layer is responsible for:

- Supplying liquidity
- Borrowing against collateral
- Repayment and withdrawals
- Pool accounting and safety checks

Primary early markets are USDC and XLM, aligned with real usage in the Stellar ecosystem.

## Why this helps

Stellar offers fast settlement and low fees. With Soroban, Zinot can add transparent lending logic on top of Stellar assets.

This helps builders and users access practical on-chain credit/liquidity flows while keeping core risk logic auditable.

## Current status

Early build phase. Contract interfaces exist and core logic is still being implemented and tested.

## Contributing

We need contributors for core pool logic, risk parameters, storage design, and test coverage.

Before opening a PR:

1. Open or pick an issue.
2. Comment with a short plan for how you will fix it.
3. Wait to be assigned by a maintainer.

Only good, passing code will be merged.

See [CONTRIBUTING.md](CONTRIBUTING.md) for full contribution rules.

## Maintainer

- m1s0g1
