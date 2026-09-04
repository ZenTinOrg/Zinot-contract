//! Extension points for custom protocol features

use soroban_sdk::{Address, Env};

pub trait ProtocolExtension {
    fn on_supply(&self, env: &Env, supplier: &Address, amount: i128);
    fn on_borrow(&self, env: &Env, borrower: &Address, amount: i128);
    fn on_liquidation(&self, env: &Env, liquidator: &Address, amount: i128);
}

pub struct ExtensionManager;

impl ExtensionManager {
    /// Register a custom extension
    pub fn register_extension(env: &Env, admin: &Address, extension: &Address) {
        admin.require_auth();
        // TODO: Store extension address
    }

    /// Call extension hooks
    pub fn call_extension_hook(env: &Env, hook_type: &str, user: &Address, amount: i128) {
        // TODO: Invoke registered extensions
    }
}
