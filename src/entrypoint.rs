use solana_program::entrypoint;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, data)?;
    Ok(())
}
