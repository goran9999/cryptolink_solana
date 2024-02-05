use std::str::FromStr;

use message_hook::{
    collect_extra_account_metas_signer_seeds, get_extra_account_metas_address_and_bump_seed,
    instruction::ProcessMessageInstruction,
};
use mv3_contract_solana::constants::MESSAGE_SEED;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList};

use crate::constants::MV3_KEY;

pub fn process_initialize_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extra_account_metas: &[ExtraAccountMeta],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let extra_account_metas_info = next_account_info(account_info_iter)?;

    let (message_key, _) = Pubkey::find_program_address(
        &[MESSAGE_SEED, program_id.as_ref()],
        &Pubkey::from_str(MV3_KEY).unwrap(),
    );

    let (expected_validation_address, bump_seed) =
        get_extra_account_metas_address_and_bump_seed(&message_key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    let bump_seed = [bump_seed];
    let signer_seeds = collect_extra_account_metas_signer_seeds(&message_key, &bump_seed);
    let length = extra_account_metas.len();
    let account_size = ExtraAccountMetaList::size_of(length)?;
    invoke_signed(
        &system_instruction::allocate(extra_account_metas_info.key, account_size as u64),
        &[extra_account_metas_info.clone()],
        &[&signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(extra_account_metas_info.key, program_id),
        &[extra_account_metas_info.clone()],
        &[&signer_seeds],
    )?;

    let mut data = extra_account_metas_info.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ProcessMessageInstruction>(&mut data, extra_account_metas)?;

    Ok(())
}
