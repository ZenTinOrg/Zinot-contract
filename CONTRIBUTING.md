# Contributing to Zinot Contract

Thanks for helping build Zinot on Stellar.

## Before you start

1. Review open issues.
2. Comment with a short fix plan (what you will change and why).
3. Wait until a maintainer assigns the issue to you.

Unassigned PRs may be closed to prevent duplicate work.

## Local workflow

1. Install Rust first (required before Stellar CLI):
   - https://rustup.rs/
2. Install Stellar CLI (macOS or Linux):
   - Script install: `curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh`
   - Homebrew (macOS or Linux): `brew install stellar-cli`
   - CLI docs: https://developers.stellar.org/docs/tools/cli
3. Install the wasm target used by Soroban:
   - rustup target add wasm32-unknown-unknown
4. Run tests before opening a PR:
   - cargo test
5. Build in release mode when needed:
   - cargo build --release

## Test setup note

- Contract methods in this repo enforce auth with require_auth.
- In unit tests, use the shared setup helper that enables mock_all_auths so smoke tests can run without manual auth payload wiring.
- If you are testing auth behavior itself, do not use global mocking; write explicit auth assertions instead.

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
