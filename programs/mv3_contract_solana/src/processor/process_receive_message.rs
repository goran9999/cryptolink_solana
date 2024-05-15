use crate::{
    constants::{
        CONFIG_SEED, MESSAGE_CLIENT_SEED, MESSAGE_CLIENT_TREASURY_SEED, MESSAGE_SEED,
        SOLANA_CHAIN_ID, TX_FEE,
    },
    error::MessengerError,
    instruction::{MessageDigest, ReceiveMessage},
    state::{
        config::{MessageClient, MessengerConfig},
        message::MessagePayload,
    },
    utils::{
        assert_account_signer, check_client_seeds, check_client_treasury_seeds,
        check_global_treasury_seeds, check_seeds, create_ecdsa_sig, initialize_account,
        pubkey_to_address, public_key_to_address,
    },
};

use borsh::BorshSerialize;
use message_hook::{get_extra_account_metas_address, onchain::invoke_execute};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
};

pub fn process_receive_message(
    receive_message: ReceiveMessage,
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let signer = next_account_info(accounts_iter)?;

    assert_account_signer(signer)?;

    let raw_config = next_account_info(accounts_iter)?;

    check_seeds(raw_config, &[CONFIG_SEED], program_id)?;

    let message_client = next_account_info(accounts_iter)?;

    check_client_seeds(receive_message.receiver, *message_client.key)?;

    let decoded_client = try_from_slice_unchecked::<MessageClient>(&message_client.data.borrow())?;

    if decoded_client.destination_contract != receive_message.receiver {
        return Err(MessengerError::InvalidClientProgramId.into());
    }

    let mut config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow())?;

    let message_data = next_account_info(accounts_iter)?;

    let client_treasury = next_account_info(accounts_iter)?;

    let treasury_bump =
        check_client_treasury_seeds(decoded_client.destination_contract, *client_treasury.key)?;

    let global_treasury = next_account_info(accounts_iter)?;

    check_global_treasury_seeds(*global_treasury.key)?;

    let system_program = next_account_info(accounts_iter)?;

    let sysvar_instructions = next_account_info(accounts_iter)?;

    let transfer_ix =
        system_instruction::transfer(client_treasury.key, global_treasury.key, TX_FEE);

    invoke_signed(
        &transfer_ix,
        &[
            client_treasury.to_owned(),
            global_treasury.to_owned(),
            system_program.to_owned(),
        ],
        &[&[
            MESSAGE_CLIENT_SEED,
            decoded_client.destination_contract.as_ref(),
            MESSAGE_CLIENT_TREASURY_SEED,
            &[treasury_bump],
        ]],
    )?;

    let bump = check_seeds(
        message_data,
        &[MESSAGE_SEED, &receive_message.tx_id.to_le_bytes()],
        program_id,
    )?;

    let data_position = if config.chainsig.is_some() { 1 } else { 0 };

    let mut encoded_recipient: Vec<u8> = [0; 12].to_vec();

    encoded_recipient
        .extend_from_slice(&hex::decode("0000000000000000000000000000000000000001").unwrap());

    let base_message_payload = MessageDigest {
        data: receive_message.data.get(data_position).unwrap().clone(),
        tx_id: receive_message.tx_id,
        sender: receive_message.sender,
        recipient: encoded_recipient.try_into().unwrap(),
        dest_chain_id: receive_message.dest_chain_id,
        source_chain_id: receive_message.source_chain_id,
    };

    let message_payload = base_message_payload.try_to_vec().unwrap();

    // role_guard(&config, signer, Role::Operator)?;

    msg!("Bridge enabled: {:?}", config.bridge_enabled);

    if !config.bridge_enabled {
        return Err(MessengerError::BrigdeNotEnabled.into());
    }

    msg!("Dest chain : {:?}", receive_message.dest_chain_id);

    if u64::from(receive_message.dest_chain_id) != SOLANA_CHAIN_ID {
        return Err(MessengerError::ChainNotSupported.into());
    }

    let mut data_index = 0;

    // let src_chain_exists = config
    //     .enabled_chains
    //     .into_iter()
    //     .any(|c| u64::from(c) == receive_message.source_chain_id);

    // if !src_chain_exists {
    //     return Err(MessengerError::ChainNotSupported.into());
    // }

    if let Some(exsig) = decoded_client.exsig {
        let exsig_vrs_bytes = receive_message
            .data
            .get(data_index)
            .expect("Missing Exsig vrs bytes!");

        if exsig_vrs_bytes.len() != 72 {
            return Err(MessengerError::InvalidSignature.into());
        }

        let recovered_signer = solana_program::secp256k1_recover::secp256k1_recover(
            &exsig_vrs_bytes[65..],
            exsig_vrs_bytes[0],
            &exsig_vrs_bytes[1..65],
        )
        .expect("Failed to recover secp256k1 sig");

        let signer = recovered_signer.0.try_to_vec().unwrap();

        if signer.as_slice() != exsig {
            return Err(MessengerError::InvalidSignature.into());
        }
        data_index = data_index + 1;
    }

    if let Some(chainsig) = config.chainsig {
        let chainsig_vrs_bytes = receive_message
            .data
            .get(data_index)
            .expect("Missing Chainsig vrs bytes!");

        let recovery_id: u8 = chainsig_vrs_bytes[64] - 27;

        let hashed = create_ecdsa_sig(&message_payload);

        let recovered_chainsig = solana_program::secp256k1_recover::secp256k1_recover(
            &hashed,
            recovery_id,
            &chainsig_vrs_bytes[..64],
        )
        .expect("Failed to recover secp256k1 sig");

        let chainsig_address = recovered_chainsig.0;

        let address = public_key_to_address(&chainsig_address);

        data_index = data_index + 1;

        if pubkey_to_address(&chainsig[12..]) != pubkey_to_address(&address) {
            return Err(MessengerError::InvalidSignature.into());
        }
    }

    if !message_data.data_is_empty() {
        return Err(MessengerError::MessageAlreadyProcessed.into());
    }

    let message_payload = receive_message.data.get(data_index).unwrap();

    let decoded_message = MessagePayload::unpack(
        receive_message.tx_id,
        receive_message.sender,
        message_payload,
    );

    initialize_account(
        signer,
        message_data,
        system_program,
        MessagePayload::LEN,
        program_id,
        &[MESSAGE_SEED, &receive_message.tx_id.to_le_bytes(), &[bump]],
    )?;

    let validate_key =
        get_extra_account_metas_address(message_client.key, &receive_message.receiver);

    let validation_key = accounts.iter().find(|acc| *acc.key == validate_key);

    if validation_key.is_none() {
        return Err(MessengerError::MissingValidationAccountInfo.into());
    }

    config.next_tx_id = config.next_tx_id.checked_add(1).unwrap();

    raw_config
        .data
        .borrow_mut()
        .copy_from_slice(&config.try_to_vec().unwrap());

    message_data
        .data
        .borrow_mut()
        .copy_from_slice(&decoded_message.try_to_vec().unwrap());

    invoke_execute(
        &receive_message.receiver,
        message_client,
        sysvar_instructions,
        accounts_iter.as_slice(),
        Vec::from(&message_payload[..]),
    )?;
    Ok(())
}
