use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

mod process_initialize_config;

use crate::instruction::V3Instruction;

pub fn process_instruction(
    data: &[u8],
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    let instruction = try_from_slice_unchecked::<V3Instruction>(data)?;

    match instruction {
        V3Instruction::ProcessIx { accountant } => {
            process_initialize_config::process_initialize_config(accountant, accounts, program_id)?;
        }
    }

    Ok(())
}
