use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{clock::Clock, pubkey::Pubkey, sysvar::Sysvar};

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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessagePayload {
    pub tx_id: u128,
    pub destination: Pubkey,
    pub received_at: i64,
    pub sender: ForeignAddress,
}

impl MessagePayload {
    pub const LEN: u64 = 16 + 32 + 8 + 32;

    pub fn unpack(tx_id: u128, sender: ForeignAddress, data: &Vec<u8>) -> Self {
        let program_id = Pubkey::new_from_array(data[..32].try_into().unwrap());

        let current_unix = Clock::get().unwrap().unix_timestamp;

        Self {
            tx_id,
            destination: program_id,
            received_at: current_unix,
            sender,
        }
    }
}
