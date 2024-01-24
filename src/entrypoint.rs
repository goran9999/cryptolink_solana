#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::processor;

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(instruction_data, accounts, program_id) {
        return Err(error);
    }

    Ok(())
}
