use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, PartialEq, PartialOrd, Clone)]
pub enum V3Instruction {
    ProcessIx { accountant: Pubkey },
}
