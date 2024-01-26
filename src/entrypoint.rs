#![cfg(all(target_os = "solana", not(feature = "no-entrypoint")))]

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::processor;

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        return Err(error);
    }

    Ok(())
}
