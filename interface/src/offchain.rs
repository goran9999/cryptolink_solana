pub use spl_tlv_account_resolution::state::{AccountDataResult, AccountFetchError};

use crate::{error::MessageHookError, instruction::ProcessMessageInstruction};
use {
    crate::{get_extra_account_metas_address, instruction::execute},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    spl_tlv_account_resolution::state::ExtraAccountMetaList,
    std::future::Future,
};

pub async fn add_extra_account_metas_for_execute<F, Fut>(
    instruction: &mut Instruction,
    program_id: &Pubkey,
    message: &Pubkey,
    data: Vec<u8>,
    fetch_account_data_fn: F,
) -> Result<(), AccountFetchError>
where
    F: Fn(Pubkey) -> Fut,
    Fut: Future<Output = AccountDataResult>,
{
    let validate_state_pubkey = get_extra_account_metas_address(message, program_id);

    let validate_state_data = fetch_account_data_fn(validate_state_pubkey)
        .await?
        .ok_or(ProgramError::InvalidAccountData)?;

    // Check to make sure the provided keys are in the instruction
    if [message]
        .iter()
        .any(|&key| !instruction.accounts.iter().any(|meta| meta.pubkey == *key))
    {
        Err(MessageHookError::IncorrectAccount)?;
    }

    let mut execute_instruction = execute(program_id, message, &validate_state_pubkey, data);

    ExtraAccountMetaList::add_to_instruction::<ProcessMessageInstruction, _, _>(
        &mut execute_instruction,
        fetch_account_data_fn,
        &validate_state_data,
    )
    .await?;

    // Add only the extra accounts resolved from the validation state
    instruction
        .accounts
        .extend_from_slice(&execute_instruction.accounts[3..]);

    // Add the program id and validation state account
    instruction
        .accounts
        .push(AccountMeta::new_readonly(*program_id, false));
    instruction
        .accounts
        .push(AccountMeta::new_readonly(validate_state_pubkey, false));

    Ok(())
}
