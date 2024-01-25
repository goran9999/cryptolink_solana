use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{instruction::ReceiveMessage, state::config::ForeignAddress};

pub fn process_receive_message(
    receive_message: ReceiveMessage,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    Ok(())
}
