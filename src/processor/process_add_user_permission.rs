use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_program::ID,
    sysvar::rent::{Rent, ID as RentPubkey},
};

use crate::{
    constants::MESSENGER_SEED,
    error::MessengerError,
    state::config::{MessengerConfig, Role, UserPermission},
    utils::{check_keys_eq, check_seeds, transfer_sol},
};

pub fn process_add_user_permission(
    role: Role,
    user: Pubkey,
    is_active: bool,
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let authority = next_account_info(accounts)?;

    let raw_config = next_account_info(accounts)?;

    check_seeds(raw_config, &[MESSENGER_SEED], program_id)?;

    let system_program = next_account_info(accounts)?;

    check_keys_eq(system_program.key, &ID)?;

    let mut config: MessengerConfig =
        try_from_slice_unchecked(&raw_config.data.borrow_mut()).unwrap();

    check_keys_eq(authority.key, &config.owner)?;

    //TODO: add custom logic for permissions per action

    let permissions = match role {
        Role::ATeam => &mut config.bridge_a_team,
        Role::Whitelist => &mut config.whitelists,
        Role::Operator => &mut config.bridge_operators,
        Role::Super => &mut config.bridge_supers,
        _ => {
            return Err(MessengerError::InvalidRoleForAction.into());
        }
    };

    if let Some(mut existing_permisson) = permissions.iter_mut().find(|perm| perm.wallet == user) {
        existing_permisson.is_active = is_active;
    } else {
        permissions.push(UserPermission {
            is_active,
            wallet: user,
        });

        let rent = next_account_info(accounts)?;

        check_keys_eq(rent.key, &RentPubkey)?;

        let realloc_fee = Rent::default().minimum_balance(UserPermission::LEN);

        transfer_sol(authority, raw_config, realloc_fee, system_program, None)?;

        raw_config.realloc(
            raw_config
                .data_len()
                .checked_add(UserPermission::LEN)
                .unwrap(),
            false,
        )?;
    }

    raw_config
        .data
        .borrow_mut()
        .copy_from_slice(&config.try_to_vec().unwrap());

    Ok(())
}
