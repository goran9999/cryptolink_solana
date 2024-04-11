use std::ops::Sub;

use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    // sysvar::instructions::load_current_index_checked,
    sysvar::Sysvar,
};

use crate::{
    instructions::{get_config_pda, get_message_pda, MessagePayload, CONFIG_SEED, MESSENGER_SEED},
    state::{Config, Message},
};

pub fn process_send_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: MessagePayload,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let config = next_account_info(accounts_iter)?;
    let message = next_account_info(accounts_iter)?;

    let sender = next_account_info(accounts_iter)?;

    let payer = next_account_info(accounts_iter)?;
    let _sysvar_instructions = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !payer.is_signer {
        return ProgramResult::Err(
            solana_program::program_error::ProgramError::MissingRequiredSignature,
        );
    }

    let (address, bump) = get_config_pda();

    if config.key.to_owned() != address {
        return ProgramResult::Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    if config.data_is_empty() {
        let new_config = Config {
            bridge_enabled: true,
            last_relayed_at: Clock::get().unwrap().unix_timestamp,
            tx_id: 0,
        };

        let balance = Rent::default().minimum_balance(Config::LEN as usize);

        let ix = system_instruction::create_account(
            payer.key,
            config.key,
            balance,
            Config::LEN,
            &crate::id(),
        );

        invoke_signed(
            &ix,
            &[
                system_program.to_owned(),
                config.to_owned(),
                payer.to_owned(),
            ],
            &[&[CONFIG_SEED, &[bump]]],
        )?;

        config
            .data
            .borrow_mut()
            .copy_from_slice(&new_config.try_to_vec().unwrap());
    }

    let (address, bump) = get_message_pda(sender.key, program_id);

    if *message.key != address {
        return ProgramResult::Err(solana_program::program_error::ProgramError::InvalidSeeds);
    }

    let mut decoded_config = try_from_slice_unchecked::<Config>(&config.data.borrow())?;

    decoded_config.tx_id = decoded_config.tx_id.checked_add(1).unwrap();

    let message_account: Message = Message {
        tx_id: decoded_config.tx_id,
        sender: *sender.key,
        destination_chain: data.destination_chain_id,
        received_at: Clock::get().unwrap().unix_timestamp,
        destination: data.destination,
        payload: data.payload,
    };

    let serialized_data = message_account.try_to_vec().unwrap();

    if message.data_is_empty() {
        let rent = Rent::default().minimum_balance(serialized_data.len());

        let create_acconut_ix = solana_program::system_instruction::create_account(
            payer.key,
            message.key,
            rent,
            serialized_data.len() as u64,
            program_id,
        );

        invoke_signed(
            &create_acconut_ix,
            &[
                payer.to_owned(),
                message.to_owned(),
                system_program.to_owned(),
            ],
            &[&[MESSENGER_SEED, sender.key.as_ref(), &[bump]]],
        )?;
    } else if message.data_len() < serialized_data.len() {
        let realloc_bytes = serialized_data.len().sub(message.data_len());

        let rent = Rent::default().minimum_balance(realloc_bytes);

        let ix = system_instruction::transfer(payer.key, message.key, rent);
        invoke(
            &ix,
            &[
                payer.to_owned(),
                message.to_owned(),
                system_program.to_owned(),
            ],
        )?;
    }

    message
        .data
        .try_borrow_mut()
        .unwrap()
        .copy_from_slice(&serialized_data);

    config
        .data
        .borrow_mut()
        .copy_from_slice(&decoded_config.try_to_vec().unwrap());

    Ok(())
}
