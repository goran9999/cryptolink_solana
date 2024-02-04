use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum TokenInstruction {
    CreateToken { name: String },
}
