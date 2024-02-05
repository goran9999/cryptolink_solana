mod error;
pub mod instruction;
pub mod onchain;

pub use solana_program;
use solana_program::pubkey::Pubkey;

const EXTRA_ACCOUNT_METAS_SEED: &[u8] = b"extra-account-metas";

pub fn get_extra_account_metas_address(mint: &Pubkey, program_id: &Pubkey) -> Pubkey {
    get_extra_account_metas_address_and_bump_seed(mint, program_id).0
}

pub fn get_extra_account_metas_address_and_bump_seed(
    message: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&collect_extra_account_metas_seeds(message), program_id)
}

pub fn collect_extra_account_metas_seeds(message: &Pubkey) -> [&[u8]; 2] {
    [EXTRA_ACCOUNT_METAS_SEED, message.as_ref()]
}

pub fn collect_extra_account_metas_signer_seeds<'a>(
    message: &'a Pubkey,
    bump_seed: &'a [u8],
) -> [&'a [u8]; 3] {
    [EXTRA_ACCOUNT_METAS_SEED, message.as_ref(), bump_seed]
}
