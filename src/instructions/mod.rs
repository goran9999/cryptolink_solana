use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken {
        name: String,
        symbol: String,
        supply: u64,
        decimals: u8,
    },
    MintToken {
        destination: Pubkey,
        amount: u64,
    },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateToken {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
    pub decimals: u8,
}
