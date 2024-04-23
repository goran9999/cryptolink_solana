pub const CONFIG_SEED: &[u8] = b"config";
pub const MESSAGE_SEED: &[u8] = b"message";
pub const MESSAGE_CLIENT_SEED: &[u8] = b"message-client";
pub const MESSAGE_CLIENT_TREASURY_SEED: &[u8] = b"message-client-treasury";
pub const GLOBAL_TREASURY: &[u8] = b"global-treasury";

//TODO: set these two once caller program is implemented
pub const CALLER_PROGRAM: &str = "";

pub const CALLER_INSTRUCTION_DISCRIMINATOR: u8 = 1;

pub const SOLANA_CHAIN_ID: u64 = 19999999991;

pub const PREFIX: &str = "\x19Ethereum Signed Message:\n";

pub const TX_FEE: u64 = 5000000;
