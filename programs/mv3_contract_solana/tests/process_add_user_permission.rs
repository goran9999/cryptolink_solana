#![cfg(feature = "test-sbf")]

mod utils;
use std::{assert, println};

use mv3_contract_solana::{
    instruction::{add_user_permission, initialize_config, AddUserPermission},
    state::config::{MessengerConfig, Role},
};
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey};

use crate::utils::ProgramTestBench;
use solana_program_test::{tokio, ProgramTestContext};
#[tokio::test]
pub async fn test_add_user_permissions() {
    let mut test = ProgramTestBench::start_impl().await;

    let ix = initialize_config(test.payer_pk, test.payer_pk, &test.program_id);

    let whitelist = Pubkey::new_unique();
    let whitelist2 = Pubkey::new_unique();
    let a_team = Pubkey::new_unique();
    let operator = Pubkey::new_unique();
    let super_1 = Pubkey::new_unique();
    let super_2 = Pubkey::new_unique();

    let ix_wl1 = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: whitelist,
            is_active: true,
            role: Role::Whitelist,
        },
    );

    let ix_a_team = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: a_team,
            is_active: true,
            role: Role::ATeam,
        },
    );

    let ix_wl2 = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: whitelist2,
            is_active: true,
            role: Role::Whitelist,
        },
    );

    let ix_op = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: operator,
            is_active: true,
            role: Role::Operator,
        },
    );

    let ix_sup = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: super_1,
            is_active: true,
            role: Role::Super,
        },
    );

    let ix_sup2 = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: super_2,
            is_active: false,
            role: Role::Super,
        },
    );

    let disable_a_team = add_user_permission(
        test.program_id,
        test.payer_pk,
        AddUserPermission {
            user: a_team,
            is_active: false,
            role: Role::ATeam,
        },
    );

    test.process_transaction(&[
        ix.clone(),
        ix_wl1,
        ix_a_team,
        ix_wl2,
        ix_op,
        ix_sup,
        ix_sup2,
        disable_a_team,
    ])
    .await
    .unwrap();

    let raw_account = test
        .client
        .get_account(ix.accounts[1].clone().pubkey)
        .await
        .unwrap()
        .unwrap();

    let config = try_from_slice_unchecked::<MessengerConfig>(&raw_account.data).unwrap();

    assert!(
        config.whitelists.get(0).unwrap().wallet == whitelist,
        "Invalid first whitelist!"
    );

    assert!(
        config.whitelists.get(1).unwrap().wallet == whitelist2,
        "Invalid first whitelist!"
    );

    assert!(
        config.bridge_a_team.get(0).unwrap().wallet == a_team
            && !config.bridge_a_team.get(0).unwrap().is_active,
        "Invalid first whitelist!"
    );

    assert!(
        config.bridge_supers.get(0).unwrap().wallet == super_1,
        "Invalid super1"
    );

    assert!(
        config.bridge_supers.get(1).unwrap().wallet == super_2
            && !config.bridge_supers.get(1).unwrap().is_active,
        "Invalid super2"
    );
}
