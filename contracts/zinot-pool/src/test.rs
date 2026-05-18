#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

/// Shared test setup for open-source contributors.
///
/// Why auth mocking is needed:
/// Contract methods call `require_auth()`, so tests must either provide
/// explicit mock auth payloads or enable global auth mocking.
fn setup() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

#[test]
fn test_supply_smoke() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let asset = Address::generate(&env);

    // Basic smoke test to ensure the interface is callable
    client.supply(&user, &asset, &100);
}
