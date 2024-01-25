use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::config::{ForeignAddress, Role};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, PartialOrd, Clone)]
pub enum V3Instruction {
    InitializeConfig {
        accountant: Pubkey,
    },
    Process {
        tx_id: u128,
        source_chain: u32,
        destination_chain: u32,
        sender: ForeignAddress,
        recipient: ForeignAddress,
        data: Vec<u8>,
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
}
