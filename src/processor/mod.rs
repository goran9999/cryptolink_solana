use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::instructions::{CreateToken, TokenInstruction};

mod process_create_token;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    data: &[u8],
) -> ProgramResult {
    let instruction = try_from_slice_unchecked::<TokenInstruction>(data)
        .expect("Instruction discriminator not found!");

    match instruction {
        TokenInstruction::CreateToken {
            name,
            symbol,
            supply,
            decimals,
        } => process_create_token::process_create_token(
            program_id,
            accounts,
            CreateToken {
                decimals,
                name,
                supply,
                symbol,
            },
        )?,
        TokenInstruction::MintToken {
            destination,
            amount,
        } => {}
    }

    Ok(())
}
