use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::{
    instructions::{CreateV1CpiBuilder, MintV1CpiBuilder},
    types::Creator,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    instruction::{check_mint_pda, check_proof_pda, get_collection_pda, NFT_SEED, TREASURY_SEED},
    state::CollectionConfig,
};

pub fn process_mint_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: Vec<u8>,
) -> ProgramResult {
    msg!("Success");
    let destination_wallet = Pubkey::try_from_slice(&data[0..32])?;

    // let uri = String::try_from_slice(&data[32..]).expect("Could not parse token URI!");

    msg!(
        "Minting NFT  to wallet {:?}",
        // uri,
        destination_wallet
    );

    let accounts_iter = &mut accounts.iter();

    let _message = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_instructions = next_account_info(accounts_iter)?;
    let _validation_info = next_account_info(accounts_iter)?;
    let treasury = next_account_info(accounts_iter)?;
    let collection_config = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let token_receiver = next_account_info(accounts_iter)?;
    let collection = next_account_info(accounts_iter)?;
    let metadata_program = next_account_info(accounts_iter)?;
    let metadata = next_account_info(accounts_iter)?;
    let edition = next_account_info(accounts_iter)?;
    let spl_ata_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token = next_account_info(accounts_iter)?;
    let token_record = next_account_info(accounts_iter)?;
    let authorization_rules = next_account_info(accounts_iter)?;
    let authorization_rules_program = next_account_info(accounts_iter)?;

    let mut decoded_data =
        try_from_slice_unchecked::<CollectionConfig>(&collection_config.data.borrow())?;

    let (_, collection_bump) = get_collection_pda();

    let (address, mint_bump) = check_mint_pda(decoded_data.total_minted);

    if address != *mint.key {
        return ProgramResult::Err(ProgramError::InvalidSeeds);
    }

    let (receiver_proof, _) = check_proof_pda(&destination_wallet, program_id.clone());

    if *collection.key != decoded_data.collection_key {
        return ProgramResult::Err(ProgramError::InvalidAccountData);
    }

    if &receiver_proof != token_receiver.key {
        return ProgramResult::Err(ProgramError::InvalidSeeds);
    }

    let name = format!(
        "{:?} #{:?}",
        decoded_data.collection_name, decoded_data.total_minted
    );

    msg!("NFT name: {:?}", name);

    let (key, treasury_bump) = Pubkey::find_program_address(&[TREASURY_SEED], program_id);

    if treasury.key != &key {
        return ProgramResult::Err(ProgramError::InvalidSeeds);
    }

    CreateV1CpiBuilder::new(metadata_program)
        .authority(collection_config)
        .collection(mpl_token_metadata::types::Collection {
            key: *collection.key,
            verified: false,
        })
        .creators(vec![Creator {
            address: decoded_data.authority,
            share: 100,
            verified: false,
        }])
        .token_standard(mpl_token_metadata::types::TokenStandard::ProgrammableNonFungible)
        .is_mutable(true)
        .master_edition(Some(edition))
        .metadata(metadata)
        .mint(mint, true)
        .name(name)
        .payer(treasury)
        .rule_set(*authorization_rules.key)
        .seller_fee_basis_points(500)
        .spl_token_program(Some(token_program))
        .system_program(system_program)
        .uri("".to_string())
        .print_supply(mpl_token_metadata::types::PrintSupply::Limited(0))
        .sysvar_instructions(sysvar_instructions)
        .update_authority(collection_config, true)
        .invoke_signed(&[
            &[NFT_SEED, &[collection_bump]],
            &[
                NFT_SEED,
                &decoded_data.total_minted.to_le_bytes(),
                &[mint_bump],
            ],
            &[TREASURY_SEED, &[treasury_bump]],
        ])?;

    MintV1CpiBuilder::new(metadata_program)
        .amount(1)
        .authority(collection_config)
        .authorization_rules(Some(authorization_rules))
        .mint(mint)
        .authorization_rules_program(Some(authorization_rules_program))
        .master_edition(Some(edition))
        .metadata(metadata)
        .payer(treasury)
        .token_record(Some(token_record))
        .spl_ata_program(spl_ata_program)
        .spl_token_program(token_program)
        .system_program(system_program)
        .token_owner(Some(token_receiver))
        .sysvar_instructions(sysvar_instructions)
        .token(token)
        .token_owner(Some(token_receiver))
        .invoke_signed(&[
            &[NFT_SEED, &[collection_bump]],
            &[TREASURY_SEED, &[treasury_bump]],
        ])?;

    decoded_data.total_minted = decoded_data.total_minted + 1;
    collection_config
        .data
        .borrow_mut()
        .copy_from_slice(&decoded_data.try_to_vec().unwrap());

    Ok(())
}
