#![cfg(feature = "test-sbf")]

mod utils;
use std::println;

use solana_program::pubkey::Pubkey;

use crate::utils::ProgramTestBench;
use solana_program_test::tokio;

#[tokio::test]
pub async fn test_create_config() {
    let test = &mut ProgramTestBench::start_impl().await;

    let ix = mv3_contract_solana::instruction::initialize_config(
        Pubkey::new_unique(),
        test.payer_pk,
        &test.program_id,
    );

    test.process_transaction(&[ix]).await;
}
