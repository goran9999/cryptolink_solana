use crate::constants::MESSENGER_SEED;
use crate::error::MessengerError;
use crate::state::config::MessengerConfig;
use crate::utils::{
    assert_account_signer, check_keys_eq, check_seeds, initialize_account, transfer_sol,
};
use borsh::BorshSerialize;
use solana_program::rent::Rent;
use solana_program::system_program::ID;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_initialize_config(
    accountant: Pubkey,
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;

    let config = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert_account_signer(payer)?;

    check_keys_eq(system_program.key, &ID)?;

    check_seeds(config, &[MESSENGER_SEED], &program_id)?;

    if !config.data_is_empty() {
        return Err(MessengerError::ConfigInitialized.into());
    }

    let new_config = MessengerConfig::new(payer.key, &accountant)
        .try_to_vec()
        .unwrap();

    let lamports = Rent::default().minimum_balance(new_config.len());

    transfer_sol(payer, config, lamports, system_program, None)?;

    initialize_account(
        payer,
        config,
        system_program,
        new_config.len() as u64,
        &program_id,
        &[MESSENGER_SEED],
    )?;

    config.data.borrow_mut().copy_from_slice(&new_config);

    Ok(())
}
