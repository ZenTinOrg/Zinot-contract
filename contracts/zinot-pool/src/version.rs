//! Contract version and compatibility

pub const CONTRACT_VERSION: &str = "0.1.0";
pub const PROTOCOL_VERSION: u32 = 1;
pub const MIN_COMPATIBLE_VERSION: u32 = 1;

pub fn get_version() -> &'static str {
    CONTRACT_VERSION
}

pub fn get_protocol_version() -> u32 {
    PROTOCOL_VERSION
}

pub fn is_compatible(other_version: u32) -> bool {
    other_version >= MIN_COMPATIBLE_VERSION && other_version <= PROTOCOL_VERSION
}
