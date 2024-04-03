use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MessengerConfig {
    pub owner: Pubkey,
    pub next_tx_id: u128,
    pub enabled_chains: Vec<u32>,
    pub whitelists: Vec<UserPermission>,
    pub bridge_enabled: bool,
    //TODO: implemented later
    pub fee_currency: Option<Pubkey>,
    pub bridge_operators: Vec<UserPermission>,
    pub bridge_supers: Vec<UserPermission>,
    pub bridge_a_team: Vec<UserPermission>,
    pub accountant: Pubkey,
    pub whitelist_only: bool,
    pub chainsig: Option<ForeignAddress>,
    pub exsig: Vec<Exsig>,
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
            exsig: vec![],
            chainsig: None,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Role {
    Operator,
    ATeam,
    Super,
    Whitelist,
    Accountant,
}

pub type ForeignAddress = [u8; 32];

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserPermission {
    pub wallet: Pubkey,
    pub is_active: bool,
}

impl UserPermission {
    pub const LEN: usize = 32 + 1;
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Exsig {
    pub recipient: Pubkey,
    pub sig: ForeignAddress,
}

impl Exsig {
    pub const LEN: usize = 8 + 32 + 32;
}
