#![cfg(feature = "test-sbf")]

use solana_program::pubkey::Pubkey;

use crate::utils::ProgramTestBench;
use std::println;

#[tokio::test]
pub async fn process_create_config() {
    let test = ProgramTestBench::start_impl().await;
    let ix = mv3_contract_solana::instruction::initialize_config(
        Pubkey::new_unique(),
        Pubkey::new_from_array(test.payer.to_bytes()),
        program_id,
    );
}
