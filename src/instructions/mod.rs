use borsh::{BorshDeserialize, BorshSerialize};
use message_hook::{get_extra_account_metas_address, instruction::MessageHookInstruction};
use mv3_contract_solana::{constants::MESSAGE_SEED, state::config::ForeignAddress};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

use crate::{constants::TOKEN_SEED, utils::get_message_pda};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken {
        name: String,
        symbol: String,
        supply: u64,
        decimals: u8,
    },

    Bridge {
        destination_chain_id: u32,
        amount: u64,
        destination: ForeignAddress,
    },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateToken {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u8,
}

pub fn create_token(
    token_name: String,
    token_symbol: String,
    supply: u64,
    decimals: u8,
    authority: Pubkey,
    mint: Pubkey,
    program_id: Pubkey,
) -> Instruction {
    let data = TokenInstruction::CreateToken {
        name: token_name,
        symbol: token_symbol,
        supply,
        decimals,
    }
    .try_to_vec()
    .unwrap();

    let (token_data, _) =
        Pubkey::find_program_address(&[TOKEN_SEED, authority.as_ref()], &program_id);

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: authority,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: token_data,
        },
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: mint,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
        AccountMeta {
            is_writable: false,
            is_signer: false,
            pubkey: spl_token::id(),
        },
        AccountMeta {
            is_writable: false,
            is_signer: false,
            pubkey: sysvar::rent::id(),
        },
    ];

    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_extra_account_meta_list(
    program_id: Pubkey,
    authority: Pubkey,
    extra_account_metas: Vec<ExtraAccountMeta>,
) -> Instruction {
    let data = MessageHookInstruction::InitializeExtraAccountMetaList {
        extra_account_metas,
    }
    .pack();

    let message = get_message_pda(&program_id);

    let extra_account_meta_key = get_extra_account_metas_address(&message, &program_id);

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta {
                is_signer: true,
                is_writable: false,
                pubkey: authority,
            },
            AccountMeta {
                is_signer: false,
                is_writable: true,
                pubkey: extra_account_meta_key,
            },
            AccountMeta {
                is_signer: false,
                is_writable: true,
                pubkey: system_program::id(),
            },
        ],
        data,
    }
}

pub fn update_extra_account_meta_list(
    program_id: Pubkey,
    authority: Pubkey,
    extra_account_metas: Vec<ExtraAccountMeta>,
) -> Instruction {
    let data = MessageHookInstruction::UpdateExtraAccountMetaList {
        extra_account_metas,
    }
    .pack();

    let (message_pda, _) = Pubkey::find_program_address(
        &[MESSAGE_SEED, program_id.as_ref()],
        &mv3_contract_solana::id(),
    );

    let extra_account_meta_address = get_extra_account_metas_address(&message_pda, &program_id);

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: authority,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: extra_account_meta_address,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: message_pda,
        },
    ];

    Instruction {
        data,
        program_id: program_id,
        accounts,
    }
}
