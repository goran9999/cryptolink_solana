use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectionConfig {
    pub collection_key: Pubkey,
    pub total_minted: u32,
    pub authority: Pubkey,
    pub collection_name: String,
    pub collection_symbol: String,
    pub total_supply: u32,
}

impl CollectionConfig {
    pub fn get_collection_config_size(name: String, symbol: String) -> usize {
        let name_len = name.as_bytes().len();
        let symbol = symbol.as_bytes().len();

        32 + 4 + 32 + name_len + symbol + 4
    }
}
