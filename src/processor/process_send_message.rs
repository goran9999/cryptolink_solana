use std::ops::Sub;

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    // sysvar::instructions::load_current_index_checked,
    sysvar::Sysvar,
};

use crate::{
    instructions::{get_message_pda, MessagePayload, MESSENGER_SEED},
    state::Message,
};

pub fn process_send_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: MessagePayload,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let message = next_account_info(accounts_iter)?;

    let sender = next_account_info(accounts_iter)?;

    let payer = next_account_info(accounts_iter)?;
    let _sysvar_instructions = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        return ProgramResult::Err(
            solana_program::program_error::ProgramError::MissingRequiredSignature,
        );
    }
    //TODO:check if we should add cpi guard
    // let current_index = load_current_index_checked(sysvar_instructions)?;

    // if current_index == 0 {
    //     return ProgramResult::Err(
    //         solana_program::program_error::ProgramError::InvalidInstructionData,
    //     );
    // }

    let (address, bump) = get_message_pda(sender.key, program_id);

    if *message.key != address {
        return ProgramResult::Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    let message_account: Message = Message {
        sender: *sender.key,
        destination_chain: data.destination_chain_id,
        received_at: Clock::get().unwrap().unix_timestamp,
        destination: data.destination,
        payload: data.payload,
    };

    let serialized_data = message_account.try_to_vec().unwrap();

    if message.data_is_empty() {
        let rent = Rent::default().minimum_balance(serialized_data.len());

        let create_acconut_ix = solana_program::system_instruction::create_account(
            payer.key,
            message.key,
            rent,
            serialized_data.len() as u64,
            program_id,
        );

        invoke_signed(
            &create_acconut_ix,
            &[
                payer.to_owned(),
                message.to_owned(),
                system_program.to_owned(),
            ],
            &[&[MESSENGER_SEED, sender.key.as_ref(), &[bump]]],
        )?;
    } else if message.data_len() < serialized_data.len() {
        let realloc_bytes = serialized_data.len().sub(message.data_len());

        let rent = Rent::default().minimum_balance(realloc_bytes);

        let ix = system_instruction::transfer(payer.key, message.key, rent);
        invoke(
            &ix,
            &[
                payer.to_owned(),
                message.to_owned(),
                system_program.to_owned(),
            ],
        )?;
    }

    message
        .data
        .try_borrow_mut()
        .unwrap()
        .copy_from_slice(&serialized_data);

    Ok(())
}
