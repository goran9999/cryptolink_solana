use borsh::BorshSerialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    system_program::ID,
};

use crate::{
    constants::MESSENGER_SEED,
    state::config::MessengerConfig,
    utils::{check_keys_eq, check_seeds, transfer_sol},
};

pub fn process_change_config(
    enabled_chains: Option<Vec<u32>>,
    bridge_enabled: Option<bool>,
    accountant: Option<Pubkey>,
    whitelist_only: Option<bool>,
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let authority = next_account_info(accounts)?;

    let raw_config = next_account_info(accounts)?;

    check_seeds(raw_config, &[MESSENGER_SEED], program_id)?;

    let mut config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow_mut())?;

    check_keys_eq(authority.key, &config.owner)?;

    let system_program = next_account_info(accounts)?;

    //TODO: add custom logic for permissions per action

    check_keys_eq(system_program.key, &ID)?;

    if let Some(new_accountant) = accountant {
        config.accountant = new_accountant;
    }

    if let Some(whitelist_only) = whitelist_only {
        config.whitelist_only = whitelist_only;
    }

    if let Some(bridge_enabled) = bridge_enabled {
        config.bridge_enabled = bridge_enabled;
    }

    if let Some(chains) = enabled_chains {
        if chains.len() > config.enabled_chains.len() {
            let new_len = (chains.len() - config.enabled_chains.len()) * 4;
            let realloc_fee = Rent::default().minimum_balance(new_len);

            transfer_sol(authority, raw_config, realloc_fee, system_program, None)?;

            raw_config.realloc(raw_config.data_len().checked_add(new_len).unwrap(), false)?;
        }

        config.enabled_chains = chains;
    }

    raw_config
        .data
        .borrow_mut()
        .serialize(&mut config.try_to_vec().unwrap())?;

    Ok(())
}
