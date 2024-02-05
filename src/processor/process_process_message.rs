use message_hook::{get_extra_account_metas_address, instruction::ProcessMessageInstruction};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

pub fn process_process_message(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    data: Vec<u8>,
) -> ProgramResult {
    let amount = u64::from_le_bytes(data.clone().try_into().unwrap());

    msg!("Mint amount: {:?}", amount);

    let accounts_iter = &mut accounts.iter();

    let message_data = next_account_info(accounts_iter)?;
    let _sysvar_instructions = next_account_info(accounts_iter)?;

    //TODO:check with sysvar instructions that ix is cpi-ed from mv3_contract

    let extra_account_metas_info = next_account_info(accounts_iter)?;

    let extra_account_meta_key = get_extra_account_metas_address(message_data.key, program_id);

    if *extra_account_metas_info.key != extra_account_meta_key {
        return Err(ProgramError::InvalidSeeds);
    }

    ExtraAccountMetaList::check_account_infos::<ProcessMessageInstruction>(
        &accounts,
        &data,
        program_id,
        &extra_account_metas_info.data.borrow(),
    )?;

    msg!("Message hook executed!");

    Ok(())
}
