use std::{fmt::Error, str::FromStr};

use mv3_contract_solana::{error::MessengerError, processor::process_instruction};
use solana_program::msg;
use solana_program::{entrypoint::ProgramResult, instruction::Instruction, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
pub struct ProgramTestBench {
    pub payer: Keypair,
    pub client: BanksClient,
    pub program_id: Pubkey,
    pub payer_pk: Pubkey,
}

impl ProgramTestBench {
    async fn create_bench(program_test: ProgramTest) -> Self {
        let test = program_test.start_with_context().await;

        let program_id = Pubkey::from_str("mv3PxTJXnsExfkFtwbKCo35fGKdtfcowo9xZsmXQ2qJ").unwrap();

        let payer_pk = test.payer.pubkey();

        ProgramTestBench {
            client: test.banks_client,
            payer: test.payer,
            program_id,
            payer_pk,
        }
    }

    pub async fn start_impl() -> ProgramTestBench {
        let mut test = ProgramTest::default();
        test.add_program(
            "mv3_contract_solana",
            Pubkey::from_str("mv3PxTJXnsExfkFtwbKCo35fGKdtfcowo9xZsmXQ2qJ").unwrap(),
            processor!(process_instruction),
        );

        Self::create_bench(test).await
    }

    pub async fn process_transaction(&mut self, instructions: &[Instruction]) -> Result<(), Error> {
        let recent_blockhash = self.client.get_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&self.payer_pk),
            &[&self.payer],
            recent_blockhash,
        );

        // let sim = self.client.simulate_transaction(tx.clone()).await.unwrap();

        // println!("SIM: {:?}", sim.result.unwrap());

        self.client
            .process_transaction_with_commitment(
                tx,
                solana_sdk::commitment_config::CommitmentLevel::Finalized,
            )
            .await
            .unwrap();

        Ok(())
    }
}
