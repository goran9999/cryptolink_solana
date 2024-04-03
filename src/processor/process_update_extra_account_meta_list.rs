use message_hook::{get_extra_account_metas_address, instruction::ProcessMessageInstruction};

use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    spl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList},
};

pub fn process_update_extra_account_meta_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    extra_account_metas: &[ExtraAccountMeta],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let _authority = next_account_info(account_info_iter)?;

    let extra_account_metas_info = next_account_info(account_info_iter)?;
    let message_info = next_account_info(account_info_iter)?;

    // Check validation account
    let expected_validation_address = get_extra_account_metas_address(message_info.key, program_id);
    if expected_validation_address != *extra_account_metas_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // Check if the extra metas have been initialized
    let min_account_size = ExtraAccountMetaList::size_of(0)?;
    let original_account_size = extra_account_metas_info.data_len();
    if program_id != extra_account_metas_info.owner || original_account_size < min_account_size {
        return Err(ProgramError::UninitializedAccount);
    }

    // If the new extra_account_metas length is different, resize the account and
    // update
    let length = extra_account_metas.len();
    let account_size = ExtraAccountMetaList::size_of(length)?;
    if account_size >= original_account_size {
        extra_account_metas_info.realloc(account_size, false)?;
        let mut data = extra_account_metas_info.try_borrow_mut_data()?;
        ExtraAccountMetaList::update::<ProcessMessageInstruction>(&mut data, extra_account_metas)?;
    } else {
        {
            let mut data = extra_account_metas_info.try_borrow_mut_data()?;
            ExtraAccountMetaList::update::<ProcessMessageInstruction>(
                &mut data,
                extra_account_metas,
            )?;
        }
        extra_account_metas_info.realloc(account_size, false)?;
    }

    Ok(())
}
