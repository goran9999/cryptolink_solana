use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Mint;

use crate::{
    constants::TOKEN_SEED,
    instructions::CreateToken,
    state::TokenData,
    utils::{check_account_signer, check_seeds, create_account},
};

pub fn process_create_token(
    program_id: &Pubkey,
    accounts: &Vec<AccountInfo>,
    data: CreateToken,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let mint_authority = next_account_info(accounts_iter)?;

    check_account_signer(mint_authority)?;

    let token_data_info = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;

    let system_program = next_account_info(accounts_iter)?;

    let token_program = next_account_info(accounts_iter)?;

    let rent = next_account_info(accounts_iter)?;

    let bump = check_seeds(
        token_data_info.key,
        &[TOKEN_SEED, mint_authority.key.as_ref()],
        program_id,
    )?;

    // if !token_data_info.data_is_empty() {
    //     return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    // }

    if !mint.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }

    let token_data = TokenData {
        authority: *mint_authority.key,
        name: data.name,
        symbol: data.symbol,
        total_bridged: 0,
        total_minted: 0,
        total_supply: data.supply,
        decimals: data.decimals,
        mint: *mint.key,
    }
    .try_to_vec()
    .unwrap();

    // create_account(
    //     mint_authority,
    //     token_data_info,
    //     system_program,
    //     token_data.len() as u64,
    //     program_id,
    //     Some(&[TOKEN_SEED, mint_authority.key.as_ref(), &[bump]]),
    // )?;

    create_account(
        mint_authority,
        mint,
        system_program,
        Mint::LEN as u64,
        token_program.key,
        None,
    )?;

    let ix = spl_token::instruction::initialize_mint(
        token_program.key,
        mint.key,
        mint_authority.key,
        Some(mint_authority.key),
        data.decimals,
    )?;

    invoke(
        &ix,
        &[
            mint_authority.clone(),
            mint.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    token_data_info
        .data
        .borrow_mut()
        .copy_from_slice(&token_data);

    Ok(())
}
