use std::ops::Sub;

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
};

use crate::{instruction::SetExsig, state::config::MessageClient, utils::transfer_sol};

pub fn process_set_exsig(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: SetExsig,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let authority = next_account_info(accounts)?;
    let message_client = next_account_info(accounts)?;
    let system_program = next_account_info(accounts)?;

    let mut decoded_client =
        try_from_slice_unchecked::<MessageClient>(&message_client.data.borrow())?;

    decoded_client.exsig = Some(data.exsig);

    if decoded_client.authority != *authority.key {
        return ProgramResult::Err(solana_program::program_error::ProgramError::IllegalOwner);
    }

    let serialized_data = decoded_client.try_to_vec().unwrap();

    if serialized_data.len() > message_client.data_len() {
        let data_diff = serialized_data.len().sub(message_client.data_len());

        let rent = Rent::default().minimum_balance(data_diff);

        transfer_sol(authority, message_client, rent, system_program, None)?;
    }

    message_client.realloc(serialized_data.len(), false)?;

    message_client
        .data
        .borrow_mut()
        .copy_from_slice(&serialized_data);

    Ok(())
}
