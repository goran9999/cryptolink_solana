use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::instructions::{
        load_current_index_checked, load_instruction_at_checked, ID as SysvarInstruction,
    },
};

use crate::{
    constants::MESSENGER_SEED,
    error::MessengerError,
    instruction::SetExsig,
    state::config::{Exsig, ForeignAddress, MessengerConfig},
    utils::{assert_account_signer, check_keys_eq, check_seeds, transfer_sol},
};

pub fn process_set_exsig(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: SetExsig,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let payer = next_account_info(accounts)?;

    assert_account_signer(payer)?;

    let raw_config = next_account_info(accounts)?;

    check_seeds(raw_config, &[MESSENGER_SEED], program_id)?;

    let mut config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow_mut())?;

    let sysvar_instructions = next_account_info(accounts)?;

    let system_program = next_account_info(accounts)?;

    check_keys_eq(sysvar_instructions.key, &SysvarInstruction)?;

    let ix_index = load_current_index_checked(sysvar_instructions)?;

    //this ix needs to be called via CPI directly from recipient program so we verify nobody unauthorized sets exsig for recipient
    let previous_ix = load_instruction_at_checked(usize::from(ix_index - 1), sysvar_instructions)?;

    let exsig_exists = config
        .exsig
        .iter()
        .any(|exisg| exisg.recipient == previous_ix.program_id);

    if exsig_exists {
        return Err(MessengerError::ExsigExists.into());
    }

    let additional_len = Exsig::LEN;

    let realloc_fee = Rent::default().minimum_balance(additional_len);

    transfer_sol(payer, raw_config, realloc_fee, system_program, None)?;

    raw_config.realloc(
        raw_config.data_len().checked_add(additional_len).unwrap(),
        false,
    )?;

    config.exsig.push(Exsig {
        recipient: previous_ix.program_id,
        sig: data.exsig,
    });

    raw_config
        .data
        .borrow_mut()
        .serialize(&mut config.try_to_vec().unwrap())?;

    Ok(())
}
