use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

use crate::instructions::{CreateToken, TokenInstruction};
use message_hook::instruction::MessageHookInstruction;
mod process_create_token;
mod process_initialize_extra_account_meta_list;
mod process_process_message;
mod process_update_extra_meta_list;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    data: &[u8],
) -> ProgramResult {
    let unpacked = MessageHookInstruction::unpack(data);

    if let Ok(message_hook_instruction) = unpacked {
        match message_hook_instruction {
            MessageHookInstruction::InitializeExtraAccountMetaList {
                extra_account_metas,
            } => {
                msg!("IX: Init extra account meta list!");

                process_initialize_extra_account_meta_list::process_initialize_extra_account_meta_list(program_id, accounts, &extra_account_metas)?
            }
            MessageHookInstruction::ProcessMessage { data } => {
                msg!("IX: Process message");

                process_process_message::process_process_message(program_id, accounts, data)?
            }
            MessageHookInstruction::UpdateExtraAccountMetaList {
                extra_account_metas,
            } => process_update_extra_meta_list::process_update_extra_account_meta_list(
                program_id,
                accounts,
                &extra_account_metas,
            )?,
        }

        return Ok(());
    }

    let instruction = try_from_slice_unchecked::<TokenInstruction>(data)
        .expect("Instruction discriminator not found!");

    match instruction {
        TokenInstruction::CreateToken {
            name,
            symbol,
            supply,
            decimals,
        } => {
            msg!("IX: Create Token");
            process_create_token::process_create_token(
                program_id,
                accounts,
                CreateToken {
                    decimals,
                    name,
                    supply,
                    symbol,
                },
            )?
        }
        TokenInstruction::Bridge {
            destination_chain_id,
            amount,
            destination,
        } => {}
    }

    Ok(())
}
