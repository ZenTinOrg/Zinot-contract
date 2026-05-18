#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_supply_smoke() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let asset = Address::generate(&env);

    // Basic smoke test to ensure the interface is callable
    client.supply(&user, &asset, &100);
}
