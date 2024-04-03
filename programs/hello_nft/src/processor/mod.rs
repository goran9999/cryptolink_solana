use message_hook::instruction::MessageHookInstruction;
use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

use crate::instruction::{HelloNftInstruction, InitCollection};

use self::init_collection::process_init_collection;

mod init_collection;
mod process_initialize_extra_account_meta;
mod process_mint_nft;
mod process_update_extra_account_meta;
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let unpacked = MessageHookInstruction::unpack(data);

    if let Ok(message_hook_instruction) = unpacked {
        match message_hook_instruction {
            MessageHookInstruction::InitializeExtraAccountMetaList {
                extra_account_metas,
            } => {
                msg!("IX: Init extra account meta list!");

                process_initialize_extra_account_meta::process_initialize_extra_account_meta_list(
                    program_id,
                    accounts,
                    &extra_account_metas,
                )?
            }
            MessageHookInstruction::ProcessMessage { data } => {
                msg!("IX: Process message");

                process_mint_nft::process_mint_nft(program_id, accounts, data)?;
            }
            MessageHookInstruction::UpdateExtraAccountMetaList {
                extra_account_metas,
            } => process_update_extra_account_meta::process_update_extra_account_meta_list(
                program_id,
                accounts,
                &extra_account_metas,
            )?,
        }

        return Ok(());
    }

    let decoded_instruction =
        try_from_slice_unchecked::<HelloNftInstruction>(data).expect("Invalid instruction!");

    match decoded_instruction {
        HelloNftInstruction::InitCollection {
            name,
            uri,
            symbol,
            total_supply,
        } => process_init_collection(
            program_id,
            accounts,
            InitCollection {
                collection_uri: uri,
                name,
                symbol,
                total_supply,
            },
        )?,
    }

    Ok(())
}
