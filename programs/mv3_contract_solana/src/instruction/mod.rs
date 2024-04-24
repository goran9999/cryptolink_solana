use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{
    constants::{CONFIG_SEED, MESSAGE_CLIENT_SEED, MESSAGE_CLIENT_TREASURY_SEED, MESSAGE_SEED},
    state::config::{ForeignAddress, MessageClient, Role},
    utils::{get_client_treasury_pda, get_global_treasury_pda, get_message_client_pda},
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
    ConfigureClient {
        authority: Pubkey,
        destination_contract: Pubkey,
        notify_on_failure: bool,
        supported_chains: Vec<u64>,
        allowed_contracts: Vec<ForeignAddress>,
        exsig: Option<ForeignAddress>,
    },
    ReceiveMessage {
        tx_id: u128,
        dest_chain_id: u64,
        receiver: Pubkey,
        data: Vec<Vec<u8>>,
        source_chain_id: u64,
        sender: ForeignAddress,
    },
    DepositWithdraw {
        action: DepositWithdraw,
        amount: u64,
    },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReceiveMessage {
    pub tx_id: u128,
    pub dest_chain_id: u64,
    pub receiver: Pubkey,
    pub data: Vec<Vec<u8>>,
    pub source_chain_id: u64,
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

pub fn receive_message(program_id: &Pubkey, data: ReceiveMessage, payer: Pubkey) -> Instruction {
    let mut accounts: Vec<AccountMeta> = vec![];

    accounts.push(AccountMeta {
        pubkey: payer,
        is_signer: true,
        is_writable: true,
    });

    let (config, _) = Pubkey::find_program_address(&[CONFIG_SEED], &program_id);

    let (client, _) = get_message_client_pda(data.receiver);

    accounts.push(AccountMeta {
        pubkey: config,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: client,
        is_signer: false,
        is_writable: false,
    });

    let client_treasury = get_client_treasury_pda(data.receiver);
    let global_treasury = get_global_treasury_pda();

    let (message, _) =
        Pubkey::find_program_address(&[MESSAGE_SEED, data.receiver.as_ref()], program_id);

    accounts.push(AccountMeta {
        pubkey: message,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: client_treasury,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: global_treasury,
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: system_program::id(),
        is_signer: false,
        is_writable: true,
    });

    accounts.push(AccountMeta {
        pubkey: sysvar::instructions::id(),
        is_signer: false,
        is_writable: false,
    });

    let mut ix_data: Vec<u8> = vec![];

    ix_data.extend_from_slice(
        &V3Instruction::ReceiveMessage {
            tx_id: data.tx_id,
            dest_chain_id: data.dest_chain_id,
            receiver: data.receiver,
            data: data.data,
            source_chain_id: data.source_chain_id,
            sender: data.sender,
        }
        .try_to_vec()
        .unwrap(),
    );

    Instruction {
        program_id: program_id.clone(),
        accounts,
        data: ix_data,
    }
}

pub fn configure_client(payer: Pubkey, data: MessageClient) -> Instruction {
    let (addr, _) = get_message_client_pda(data.destination_contract);

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: addr,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: data.destination_contract,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
    ];

    let data = V3Instruction::ConfigureClient {
        authority: data.authority,
        destination_contract: data.destination_contract,
        notify_on_failure: data.notify_on_failure,
        supported_chains: data.supported_chains,
        allowed_contracts: data.allowed_contracts,
        exsig: data.exsig,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        accounts,
        data,
        program_id: crate::id(),
    };

    ix
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessageDigest {
    pub data: Vec<u8>,
    pub tx_id: u128,
    pub sender: ForeignAddress,
    pub recipient: Pubkey,
    pub dest_chain_id: u64,
    pub source_chain_id: u64,
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, PartialOrd, Clone)]
pub enum DepositWithdraw {
    Deposit,
    Withdraw,
}

pub fn deposit_withdraw_sol(
    payer: Pubkey,
    destination_contract: Pubkey,
    amount: u64,
    action: DepositWithdraw,
) -> Instruction {
    let (address, _) = get_message_client_pda(destination_contract);

    let (treasury, _) = Pubkey::find_program_address(
        &[
            MESSAGE_CLIENT_SEED,
            destination_contract.as_ref(),
            MESSAGE_CLIENT_TREASURY_SEED,
        ],
        &crate::id(),
    );

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: address,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: treasury,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
    ];

    let data = V3Instruction::DepositWithdraw { action, amount }
        .try_to_vec()
        .unwrap();

    Instruction {
        program_id: crate::id(),
        accounts,
        data,
    }
}
