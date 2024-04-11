use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub type ForeignAddress = [u8; 32];

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Message {
    pub sender: Pubkey,
    pub destination_chain: u64,
    pub received_at: i64,
    pub destination: ForeignAddress,
    pub payload: Vec<u8>,
    pub tx_id: u128,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Config {
    pub tx_id: u128,
    pub last_relayed_at: i64,
    pub bridge_enabled: bool,
}

impl Config {
    pub const LEN: u64 = 16 + 8 + 1;
}
