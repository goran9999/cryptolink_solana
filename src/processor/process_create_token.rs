use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
};

use crate::{
    constants::TOKEN_SEED,
    instructions::CreateToken,
    state::TokenData,
    utils::{check_account_signer, check_seeds, create_account, transfer_sol},
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

    let bump = check_seeds(
        token_data_info.key,
        &[TOKEN_SEED, mint_authority.key.as_ref()],
        program_id,
    )?;

    if !token_data_info.data_is_empty() {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }

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
    }
    .try_to_vec()
    .unwrap();

    let rent = Rent::default().minimum_balance(token_data.len());

    transfer_sol(mint_authority, token_data_info, system_program, rent)?;

    create_account(
        mint_authority,
        token_data_info,
        system_program,
        token_data.len() as u64,
        program_id,
        &[TOKEN_SEED, mint_authority.key.as_ref(), &[bump]],
    )?;

    spl_token::instruction::initialize_mint(
        token_program.key,
        mint.key,
        mint_authority.key,
        Some(mint_authority.key),
        data.decimals,
    )?;

    token_data_info
        .data
        .borrow_mut()
        .copy_from_slice(&token_data);

    Ok(())
}
