use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    system_program,
};

use crate::{
    constants::CONFIG_SEED,
    state::config::{ForeignAddress, Role},
};

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

pub fn initialize_config(accountant: Pubkey, payer: Pubkey, program_id: &Pubkey) -> Instruction {
    let mut accounts: Vec<AccountMeta> = vec![];

    let mut data: Vec<u8> = vec![];

    accounts.push(AccountMeta {
        pubkey: payer,
        is_signer: true,
        is_writable: true,
    });

    let (config, _) = Pubkey::find_program_address(&[CONFIG_SEED], program_id);

    accounts.push(AccountMeta {
        pubkey: config,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false,
    });

    data.extend_from_slice(
        &V3Instruction::InitializeConfig { accountant }
            .try_to_vec()
            .unwrap(),
    );

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

pub fn change_config(program_id: Pubkey, payer: Pubkey, data: ChangeConfig) -> Instruction {
    let mut accounts: Vec<AccountMeta> = vec![];

    accounts.push(AccountMeta {
        pubkey: payer,
        is_signer: true,
        is_writable: true,
    });

    let (config, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);

    accounts.push(AccountMeta {
        pubkey: config,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false,
    });

    let mut ix_data: Vec<u8> = vec![];

    ix_data.extend_from_slice(
        &V3Instruction::ChangeConfig {
            enabled_chains: data.enabled_chains,
            bridge_enabled: data.bridge_enabled,
            accountant: data.accountant,
            whitelist_only: data.whitelist_only,
            chainsig: data.chainsig,
        }
        .try_to_vec()
        .unwrap(),
    );

    Instruction {
        program_id,
        accounts,
        data: ix_data,
    }
}

pub fn add_user_permission(
    program_id: Pubkey,
    payer: Pubkey,
    data: AddUserPermission,
) -> Instruction {
    let mut accounts: Vec<AccountMeta> = vec![];

    accounts.push(AccountMeta {
        pubkey: payer,
        is_signer: true,
        is_writable: true,
    });

    let (config, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);

    accounts.push(AccountMeta {
        pubkey: config,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: false,
    });

    accounts.push(AccountMeta {
        pubkey: solana_program::sysvar::rent::id(),
        is_signer: false,
        is_writable: false,
    });

    let mut ix_data: Vec<u8> = vec![];

    ix_data.extend_from_slice(
        &V3Instruction::AddUserPermission {
            user: data.user,
            is_active: data.is_active,
            role: data.role,
        }
        .try_to_vec()
        .unwrap(),
    );

    Instruction {
        program_id,
        accounts,
        data: ix_data,
    }
}
