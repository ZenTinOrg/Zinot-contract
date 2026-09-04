#![cfg(test)]

//! Testing utilities for contract tests

use soroban_sdk::{Address, Env};

pub fn create_test_env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

pub fn generate_test_address(env: &Env) -> Address {
    Address::generate(env)
}

pub fn generate_test_asset(env: &Env) -> Address {
    Address::generate(env)
}
