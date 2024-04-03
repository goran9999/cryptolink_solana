use borsh::BorshSerialize;
use mpl_token_metadata::{
    instructions::{CreateCpiBuilder, MintV1Cpi, MintV1CpiAccounts, MintV1InstructionArgs},
    types::{CollectionDetails, CreateArgs, Creator},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

use crate::{
    instruction::{get_collection_pda, InitCollection, NFT_SEED},
    state::CollectionConfig,
};

pub fn process_init_collection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: InitCollection,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority = next_account_info(accounts_iter)?;

    if !authority.is_signer {
        return ProgramResult::Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump) = get_collection_pda();

    let collection_config = next_account_info(accounts_iter)?;

    if pda != *collection_config.key {
        return ProgramResult::Err(ProgramError::InvalidSeeds);
    }

    if !collection_config.data_is_empty() {
        return ProgramResult::Err(ProgramError::AccountAlreadyInitialized);
    }

    let collection = next_account_info(accounts_iter)?;

    let config_data: CollectionConfig = CollectionConfig {
        authority: *authority.key,
        collection_key: *collection.key,
        collection_name: data.name.clone(),
        collection_symbol: data.symbol.clone(),
        total_minted: 0,
        total_supply: data.total_supply,
    };

    let size = config_data.try_to_vec().unwrap().len();
    let rent = Rent::default().minimum_balance(CollectionConfig::get_collection_config_size(
        data.name.clone(),
        data.symbol.clone(),
    ));

    msg!("Account size: {:?}", size);

    // collection_config.realloc(size, false)?;

    let create_acc = system_instruction::create_account(
        authority.key,
        collection_config.key,
        rent,
        size as u64,
        &crate::id(),
    );

    invoke_signed(
        &create_acc,
        &[authority.clone(), collection_config.clone()],
        &[&[NFT_SEED, &[bump]]],
    )?;
    let ix = system_instruction::transfer(authority.key, collection_config.key, rent);

    invoke(&ix, &[authority.clone(), collection_config.clone()])?;

    let metadata = next_account_info(accounts_iter)?;
    let edition = next_account_info(accounts_iter)?;
    let token_record = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let auth_rules = next_account_info(accounts_iter)?;
    let auth_rules_program = next_account_info(accounts_iter)?;
    let mpl_program = next_account_info(accounts_iter)?;
    let sysvar_instructions = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let spl_ata_program = next_account_info(accounts_iter)?;

    CreateCpiBuilder::new(mpl_program)
        .authority(collection_config)
        .master_edition(Some(edition))
        .metadata(metadata)
        .mint(collection, true)
        .payer(authority)
        .update_authority(collection_config, true)
        .sysvar_instructions(sysvar_instructions)
        .spl_token_program(Some(token_program))
        .system_program(system_program)
        .create_args(CreateArgs::V1 {
            name: data.name.clone(),
            symbol: data.symbol.clone(),
            uri: data.collection_uri.clone(),
            seller_fee_basis_points: 500,
            creators: Some(vec![Creator {
                address: *authority.key,
                share: 100,
                verified: false,
            }]),
            primary_sale_happened: false,
            is_mutable: true,
            token_standard: mpl_token_metadata::types::TokenStandard::ProgrammableNonFungible,
            collection: None,
            uses: None,
            collection_details: Some(CollectionDetails::V1 {
                size: u64::from(data.total_supply),
            }),
            rule_set: Some(auth_rules.key.clone()),
            decimals: None,
            print_supply: Some(mpl_token_metadata::types::PrintSupply::Limited(0)),
        })
        .invoke_signed(&[&[NFT_SEED, &[bump]]])?;

    MintV1Cpi::new(
        mpl_program,
        MintV1CpiAccounts {
            authority: collection_config,
            authorization_rules: Some(auth_rules),
            authorization_rules_program: Some(auth_rules_program),
            delegate_record: None,
            master_edition: Some(edition),
            metadata,
            mint: collection,
            payer: authority,
            spl_token_program: token_program,
            token: token_account,
            system_program,
            sysvar_instructions,
            token_owner: Some(authority),
            token_record: Some(token_record),
            spl_ata_program,
        },
        MintV1InstructionArgs {
            amount: 1,
            authorization_data: None,
        },
    );
    // .invoke_signed(&[&[NFT_SEED, &[bump]]])?;

    collection_config
        .data
        .borrow_mut()
        .copy_from_slice(&config_data.try_to_vec().unwrap());

    Ok(())
}
