use std::ops::Add;

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

use crate::{
    constants::MESSAGE_CLIENT_SEED, error::MessengerError, state::config::MessageClient,
    utils::get_message_client_pda,
};

pub fn process_configure_client(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: MessageClient,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature.into());
    }

    let message_client = next_account_info(accounts_iter)?;

    let program = next_account_info(accounts_iter)?;

    if *program.key != data.destination_contract || !program.executable {
        return Err(MessengerError::InvalidClientProgramId.into());
    }

    let (pda, bump) = get_message_client_pda(data.destination_contract);

    if *message_client.key != pda {
        return Err(solana_program::program_error::ProgramError::InvalidSeeds.into());
    }

    let system_program = next_account_info(accounts_iter)?;

    if message_client.data_is_empty() {
        let ix = system_instruction::create_account(
            payer.key,
            message_client.key,
            Rent::default().minimum_balance(MessageClient::LEN as usize),
            MessageClient::LEN,
            &program_id,
        );

        invoke_signed(
            &ix,
            &[
                payer.to_owned(),
                message_client.to_owned(),
                system_program.to_owned(),
            ],
            &[&[
                MESSAGE_CLIENT_SEED,
                data.destination_contract.as_ref(),
                &[bump],
            ]],
        )?;
    }

    //TODO:add check for program update authority

    let data_diff = data
        .try_to_vec()
        .unwrap()
        .len()
        .checked_sub(message_client.data_len())
        .unwrap();

    if data_diff > 0 {
        message_client.realloc(message_client.data_len().add(data_diff), false)?;
    }

    message_client
        .data
        .borrow_mut()
        .copy_from_slice(&data.try_to_vec().unwrap());

    Ok(())
}
