# Contributing to Zinot Contract

Thanks for helping build Zinot on Stellar.

## Before you start
1. Review open issues.
2. Comment with a short fix plan (what you will change and why).
3. Wait until a maintainer assigns the issue to you.

Unassigned PRs may be closed to prevent duplicate work.

## Local workflow
1. Run tests before opening a PR:
   - cargo test
2. Build in release mode when needed:
   - cargo build --release

## PR requirements
1. Keep changes focused and minimal.
2. Include tests for logic changes.
3. Explain any storage/key/risk-model changes clearly.
4. Ensure all checks pass.

Only good, passing code will be merged.

## Contract direction (must align)
- Soroban contract is the source of truth for state.
- USDC and XLM are the first-class market assets.
- Collateral, debt, and health checks must be explicit and test-covered.
- Safety over speed: no risky shortcuts in financial logic.

## Assignment and review
Maintainer: m1s0g1

I will assign on time and merge on time. Do be patient and just make your PRs.
