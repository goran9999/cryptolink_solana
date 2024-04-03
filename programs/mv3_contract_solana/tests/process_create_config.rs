#![cfg(feature = "test-sbf")]

mod utils;
use std::assert;

use mv3_contract_solana::state::config::MessengerConfig;
use solana_program::{borsh0_10::try_from_slice_unchecked, pubkey::Pubkey};

use crate::utils::ProgramTestBench;
use solana_program_test::tokio;

#[tokio::test]
pub async fn test_create_config() {
    let test = &mut ProgramTestBench::start_impl().await;

    let accountant = Pubkey::new_unique();

    let ix = mv3_contract_solana::instruction::initialize_config(
        accountant,
        test.payer_pk,
        &test.program_id,
    );
    test.process_transaction(&[ix.clone()]).await.unwrap();

    let acc = ix.accounts[1].clone();

    let raw_config_info = test.client.get_account(acc.pubkey).await.unwrap().unwrap();

    let config = try_from_slice_unchecked::<MessengerConfig>(&raw_config_info.data).unwrap();

    assert!(
        config.accountant == accountant,
        "Invalid accountant address!"
    );

    assert!(config.owner == test.payer_pk, "Invalid owner PK!");

    assert!(config.bridge_enabled, "Bridge not enabled!");

    assert!(!config.whitelist_only, "Whitelist enabled!");
}
