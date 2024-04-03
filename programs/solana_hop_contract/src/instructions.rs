use borsh::{BorshDeserialize, BorshSerialize};
use message_hook::{get_extra_account_metas_address, instruction::MessageHookInstruction};
use mv3_contract_solana::{constants::MESSAGE_SEED, utils::get_message_pda};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct HopData {
    pub source_chain: Vec<u8>,
    pub hop: Vec<u8>,
    pub chainlist: Vec<u8>,
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
