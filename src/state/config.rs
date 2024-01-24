use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessengerConfig {
    pub owner: Pubkey,
    pub next_tx_id: u128,
    pub enabled_chains: Vec<u32>,
    pub whitelists: Vec<Pubkey>,
    pub bridge_enabled: bool,
    //TODO: implemented later
    pub fee_currency: Option<Pubkey>,
    pub bridge_operators: Vec<Pubkey>,
    pub bridge_supers: Vec<Pubkey>,
    pub bridge_a_team: Vec<Pubkey>,
    pub accountant: Pubkey,
    pub whitelist_only: bool,
}

impl MessengerConfig {
    pub fn new(owner: &Pubkey, accountant: &Pubkey) -> Self {
        MessengerConfig {
            owner: owner.clone(),
            next_tx_id: 0_u128,
            enabled_chains: vec![],
            whitelists: vec![],
            bridge_enabled: true,
            fee_currency: None,
            bridge_operators: vec![],
            bridge_supers: vec![],
            bridge_a_team: vec![],
            accountant: accountant.clone(),
            whitelist_only: false,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Role {
    Operator,
    ATeam,
    Super,
    Whitelist,
    Accountant,
}

pub type ForeignAddress = [u8; 32];
