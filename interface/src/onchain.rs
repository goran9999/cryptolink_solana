use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

use crate::{error::MessageHookError, get_extra_account_metas_address, instruction};

pub fn invoke_execute<'a>(
    program_id: &Pubkey,
    message: &AccountInfo<'a>,
    sysvar_instructions: &AccountInfo<'a>,
    additional_accounts: &[AccountInfo<'a>],
    data: Vec<u8>,
) -> ProgramResult {
    let validate_pubkey = get_extra_account_metas_address(message.key, program_id);

    let validate_account_info = additional_accounts
        .iter()
        .find(|acc| *acc.key == validate_pubkey)
        .ok_or(MessageHookError::IncorrectAccount)?;

    let mut cpi_instruction = instruction::execute(program_id, message.key, &validate_pubkey, data);

    let mut cpi_account_infos = vec![
        message.clone(),
        sysvar_instructions.clone(),
        validate_account_info.clone(),
    ];

    ExtraAccountMetaList::add_to_cpi_instruction::<instruction::ProcessMessageInstruction>(
        &mut cpi_instruction,
        &mut cpi_account_infos,
        &validate_account_info.try_borrow_data()?,
        additional_accounts,
    )?;

    invoke(&cpi_instruction, &cpi_account_infos)?;

    Ok(())
}
