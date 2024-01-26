use std::{fmt::Error, str::FromStr};

use mv3_contract_solana::processor::process_instruction;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, transaction::Transaction};

pub struct ProgramTestBench {
    pub payer: Keypair,
    pub client: BanksClient,
}

impl ProgramTestBench {
    pub async fn create_bench(program_test: ProgramTest) -> Self {
        let test = program_test.start_with_context().await;

        ProgramTestBench {
            client: test.banks_client,
            payer: test.payer,
        }
    }

    pub async fn start_impl() -> ProgramTestBench {
        let mut test = ProgramTest::default();
        test.add_program(
            "mv3_contract_solana",
            Pubkey::from_str("s").unwrap(),
            processor!(process_instruction),
        );

        Self::create_bench(test).await
    }

    pub async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
        fee_payer: &Pubkey,
    ) -> Result<(), Error> {
        let recent_blockhash = self.client.get_latest_blockhash().await.unwrap();

        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(fee_payer),
            signers,
            recent_blockhash,
        );

        let sim = self.client.simulate_transaction(tx.clone()).await.unwrap();

        println!("SIM:{:?}", sim.result.unwrap());

        self.client.send_transaction(tx).await.unwrap();

        Ok(())
    }
}
