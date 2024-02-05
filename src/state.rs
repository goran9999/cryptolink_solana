use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, borsh::BorshSerialize)]
pub struct TokenData {
    pub authority: Pubkey,
    pub total_supply: u64,
    pub mint: Pubkey,
    pub total_minted: u64,
    pub total_bridged: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
