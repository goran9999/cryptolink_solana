use borsh::BorshSerialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    rent::Rent,
    system_program::ID,
};

use crate::{
    constants::CONFIG_SEED,
    instruction::ChangeConfig,
    state::config::MessengerConfig,
    utils::{check_keys_eq, check_seeds, transfer_sol},
};

pub fn process_change_config(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    data: ChangeConfig,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let authority = next_account_info(accounts)?;

    let raw_config = next_account_info(accounts)?;

    msg!("Config len {}", raw_config.data_len());

    check_seeds(raw_config, &[CONFIG_SEED], program_id)?;

    let mut config: Box<MessengerConfig> = Box::new(try_from_slice_unchecked::<MessengerConfig>(
        &raw_config.data.borrow_mut(),
    )?);

    msg!("Config {:?}", config);

    if *authority.key != config.owner {
        return ProgramResult::Err(solana_program::program_error::ProgramError::IllegalOwner);
    }

    check_keys_eq(authority.key, &config.owner)?;

    let system_program = next_account_info(accounts)?;

    //TODO: add custom logic for permissions per action

    check_keys_eq(system_program.key, &ID)?;

    if let Some(new_accountant) = data.accountant {
        config.accountant = new_accountant;
    }

    if let Some(whitelist_only) = data.whitelist_only {
        config.whitelist_only = whitelist_only;
    }

    if let Some(bridge_enabled) = data.bridge_enabled {
        config.bridge_enabled = bridge_enabled;
    }

    if let Some(chains) = data.enabled_chains {
        config.enabled_chains = chains;
    }

    config.chainsig = data.chainsig;

    match config
        .try_to_vec()
        .unwrap()
        .len()
        .checked_sub(raw_config.data_len())
    {
        Some(len_diff) => {
            msg!("Len diff {}", len_diff);
            let additional_rent = Rent::default().minimum_balance(len_diff);

            transfer_sol(authority, raw_config, additional_rent, system_program, None)?;
        }
        None => {}
    }
    raw_config.realloc(config.try_to_vec().unwrap().len(), false)?;

    raw_config
        .data
        .borrow_mut()
        .copy_from_slice(&config.try_to_vec().unwrap());

    Ok(())
}
