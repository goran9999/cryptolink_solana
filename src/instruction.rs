use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use message_hook::{get_extra_account_metas_address, instruction::MessageHookInstruction};
use mpl_token_auth_rules::ID as MPL_TOKEN_AUTH;
use mpl_token_metadata::ID as MPL_TOKEN_METADATA;

use mv3_contract_solana::{constants::MESSAGE_SEED, utils::get_message_pda};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum HelloNftInstruction {
    InitCollection {
        name: String,
        uri: String,
        symbol: String,
        total_supply: u32,
    },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitCollection {
    pub name: String,
    pub symbol: String,
    pub total_supply: u32,
    pub collection_uri: String,
}

pub const NFT_SEED: &[u8] = b"nft";
pub const TREASURY_SEED: &[u8] = b"treasury";

pub fn get_collection_pda() -> (Pubkey, u8) {
    let (key, bump) = Pubkey::find_program_address(&[NFT_SEED], &crate::id());

    (key, bump)
}

pub fn check_mint_pda(total_minted: u32) -> (Pubkey, u8) {
    let address =
        Pubkey::find_program_address(&[NFT_SEED, &total_minted.to_le_bytes()], &crate::id());

    address
}

pub fn check_proof_pda(receiver: &Pubkey, program_id: Pubkey) -> (Pubkey, u8) {
    let address = Pubkey::find_program_address(&[b"nft_claim", receiver.as_ref()], &program_id);

    address
}

pub fn init_collection(
    program_id: Pubkey,
    authority: &Pubkey,
    collection: &Pubkey,
    metadata: &Pubkey,
    edition: &Pubkey,
    token_record: &Pubkey,
    authorization_rules: &Pubkey,
    token_account: &Pubkey,
    collection_config: InitCollection,
) -> Instruction {
    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: authority.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: get_collection_pda().0,
        },
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: collection.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: metadata.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: edition.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: token_record.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: token_account.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: authorization_rules.clone(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: MPL_TOKEN_AUTH,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: MPL_TOKEN_METADATA,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: sysvar::instructions::id(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: spl_token::id(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        },
    ];
    let mut data: Vec<u8> = vec![];

    data.extend_from_slice(
        &HelloNftInstruction::InitCollection {
            name: collection_config.name,
            uri: collection_config.collection_uri,
            symbol: collection_config.symbol,
            total_supply: collection_config.total_supply,
        }
        .try_to_vec()
        .unwrap(),
    );

    let ix: Instruction = Instruction {
        program_id,
        accounts,
        data,
    };

    ix
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
