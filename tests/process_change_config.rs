#![cfg(feature = "test-sbf")]

mod utils;
use std::option::Option::Some;
use std::{assert, println};

use mv3_contract_solana::instruction::{change_config, initialize_config, ChangeConfig};
use mv3_contract_solana::state::config::MessengerConfig;
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey};

use crate::utils::ProgramTestBench;
use solana_program_test::tokio;
#[tokio::test]
pub async fn test_change_config() {
    let mut test = ProgramTestBench::start_impl().await;

    let init_ix = initialize_config(test.payer_pk, test.payer_pk, &test.program_id);

    let new_accountant = Pubkey::new_unique();

    let modify_ix = change_config(
        test.program_id,
        test.payer_pk,
        ChangeConfig {
            enabled_chains: Some(vec![2, 3, 4]),
            bridge_enabled: None,
            accountant: Some(new_accountant),
            whitelist_only: Some(true),
            chainsig: None,
        },
    );

    test.process_transaction(&[init_ix.clone(), modify_ix])
        .await
        .unwrap();

    let raw_account = test
        .client
        .get_account(init_ix.accounts[1].pubkey.clone())
        .await
        .unwrap()
        .unwrap();

    let config = try_from_slice_unchecked::<MessengerConfig>(&raw_account.data).unwrap();

    assert!(config.accountant == new_accountant, "Invalid accountant!");

    assert!(config.enabled_chains == [2, 3, 4], "Invalid chains!");

    assert!(config.whitelist_only, "Whitelist not enabled!");

    assert!(config.bridge_enabled, "Bridge in invalid state!");
}
