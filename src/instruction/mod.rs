use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::config::{ForeignAddress, Role};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, PartialOrd, Clone)]
pub enum V3Instruction {
    InitializeConfig {
        accountant: Pubkey,
    },

    AddUserPermission {
        user: Pubkey,
        is_active: bool,
        role: Role,
    },
    ChangeConfig {
        enabled_chains: Option<Vec<u32>>,
        bridge_enabled: Option<bool>,
        accountant: Option<Pubkey>,
        whitelist_only: Option<bool>,
        chainsig: Option<ForeignAddress>,
    },
    Send {
        recipient: ForeignAddress,
        chain: u32,
        confirmations: u16,
        data: Vec<u8>,
    },
    SetExsig {
        exsig: ForeignAddress,
    },

    ReceiveMessage {
        tx_id: u128,
        dest_chain_id: u32,
        receiver: Pubkey,
        data: Vec<Vec<u8>>,
        source_chain_id: u32,
        sender: ForeignAddress,
    },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReceiveMessage {
    pub tx_id: u128,
    pub dest_chain_id: u32,
    pub receiver: Pubkey,
    pub data: Vec<Vec<u8>>,
    pub source_chain_id: u32,
    pub sender: ForeignAddress,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SetExsig {
    pub exsig: ForeignAddress,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SendMessage {
    pub recipient: ForeignAddress,
    pub chain: u32,
    pub confirmations: u16,
    pub data: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ChangeConfig {
    pub enabled_chains: Option<Vec<u32>>,
    pub bridge_enabled: Option<bool>,
    pub accountant: Option<Pubkey>,
    pub whitelist_only: Option<bool>,
    pub chainsig: Option<ForeignAddress>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeConfig {
    pub accountant: Pubkey,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AddUserPermission {
    pub user: Pubkey,
    pub is_active: bool,
    pub role: Role,
}
