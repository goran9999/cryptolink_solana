use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::instructions::{MessagePayload, Mv3SenderInstruction};

use self::process_send_message::process_send_message;

mod process_send_message;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = try_from_slice_unchecked::<Mv3SenderInstruction>(data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidAccountData)
        .unwrap();

    match instruction {
        Mv3SenderInstruction::SendData {
            destination_chain_id,
            payload,
            destination,
        } => process_send_message(
            program_id,
            accounts,
            MessagePayload {
                destination_chain_id,
                payload,
                destination,
            },
        )?,
    }

    Ok(())
}
