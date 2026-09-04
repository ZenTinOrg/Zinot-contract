#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

fn setup() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

#[test]
fn test_init() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);
}

#[test]
fn test_get_balance() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let asset = Address::generate(&env);

    let balance = client.get_balance(&user, &asset);
    assert_eq!(balance, 0);
}

#[test]
fn test_get_debt() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let asset = Address::generate(&env);

    let debt = client.get_debt(&user, &asset);
    assert_eq!(debt, 0);
}

#[test]
fn test_get_total_liquidity() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let asset = Address::generate(&env);

    let liquidity = client.get_total_liquidity(&asset);
    assert_eq!(liquidity, 0);
}

#[test]
fn test_get_total_borrowed() {
    let env = setup();
    let contract_id = env.register_contract(None, ZinotPool);
    let client = ZinotPoolClient::new(&env, &contract_id);

    let asset = Address::generate(&env);

    let borrowed = client.get_total_borrowed(&asset);
    assert_eq!(borrowed, 0);
}
