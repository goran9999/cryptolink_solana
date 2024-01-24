use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use super::config::ForeignAddress;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Message {
    pub tx_id: u128,
    pub sender: Pubkey,
    pub recipient: ForeignAddress,
    pub chain: u32,
    pub data: Vec<u8>,
    pub confirmations: u16,
}

impl Message {
    pub const LEN: usize = 4 + 16 + 32 + 32 + 4 + 4 + 2;
}
