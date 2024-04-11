use crate::{
    constants::{CONFIG_SEED, MESSAGE_SEED, SOLANA_CHAIN_ID},
    error::MessengerError,
    instruction::ReceiveMessage,
    state::{
        config::{MessengerConfig, Role},
        message::MessagePayload,
    },
    utils::{
        assert_account_signer, check_seeds, initialize_account, public_key_to_address, role_guard,
    },
};
use borsh::BorshSerialize;
use message_hook::{get_extra_account_metas_address, onchain::invoke_execute};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
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

    let config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow())?;

    let message_data = next_account_info(accounts_iter)?;

    let system_program = next_account_info(accounts_iter)?;

    let sysvar_instructions = next_account_info(accounts_iter)?;

    let bump = check_seeds(
        message_data,
        &[MESSAGE_SEED, receive_message.receiver.as_ref()],
        program_id,
    )?;

    role_guard(&config, signer, Role::Operator)?;

    msg!("Bridge enabled: {:?}", config.bridge_enabled);

    if !config.bridge_enabled {
        return Err(MessengerError::BrigdeNotEnabled.into());
    }

    msg!("Dest chain : {:?}", receive_message.dest_chain_id);

    if receive_message.dest_chain_id != SOLANA_CHAIN_ID {
        return Err(MessengerError::ChainNotSupported.into());
    }

    let mut data_index = 0;

    let src_chain_exists = config
        .enabled_chains
        .into_iter()
        .any(|c| c == receive_message.source_chain_id);

    if !src_chain_exists {
        return Err(MessengerError::ChainNotSupported.into());
    }

    if let Some(exsig) = config
        .exsig
        .iter()
        .find(|exsig| exsig.recipient == receive_message.receiver)
    {
        let exsig_vrs_bytes = receive_message
            .data
            .get(data_index)
            .expect("Missing Exsig vrs bytes!");

        if exsig_vrs_bytes.len() != 65 {
            return Err(MessengerError::InvalidSignature.into());
        }

        let recovered_signer = solana_program::secp256k1_recover::secp256k1_recover(
            &exsig_vrs_bytes[65..],
            exsig_vrs_bytes[0],
            &exsig_vrs_bytes[1..65],
        )
        .expect("Failed to recover secp256k1 sig");

        let signer = recovered_signer.0.try_to_vec().unwrap();

        if signer.as_slice() != exsig.sig {
            return Err(MessengerError::InvalidSignature.into());
        }
        data_index = data_index + 1;
    }

    if let Some(chainsig) = config.chainsig {
        let chainsig_vrs_bytes = receive_message
            .data
            .get(data_index)
            .expect("Missing Chainsig vrs bytes!");

        let recovered_chainsig = solana_program::secp256k1_recover::secp256k1_recover(
            &chainsig_vrs_bytes[65..],
            chainsig_vrs_bytes[64] - 27,
            &chainsig_vrs_bytes[..64],
        )
        .expect("Failed to recover secp256k1 sig");

        let chainsig_address = recovered_chainsig.0;

        let address = public_key_to_address(&chainsig_address);

        data_index = data_index + 1;

        if address != chainsig[12..] {
            return Err(MessengerError::InvalidSignature.into());
        }
    }

    let message_payload = receive_message.data.get(data_index).unwrap();

    let decoded_message = MessagePayload::unpack(
        receive_message.tx_id,
        receive_message.sender,
        message_payload,
    );

    if message_data.data_is_empty() {
        initialize_account(
            signer,
            message_data,
            system_program,
            MessagePayload::LEN,
            program_id,
            &[MESSAGE_SEED, decoded_message.destination.as_ref(), &[bump]],
        )?;
    }

    let validate_key =
        get_extra_account_metas_address(message_data.key, &decoded_message.destination);

    let validation_key = accounts.iter().find(|acc| *acc.key == validate_key);

    if validation_key.is_none() {
        return Err(MessengerError::MissingValidationAccountInfo.into());
    }

    message_data
        .data
        .borrow_mut()
        .copy_from_slice(&decoded_message.try_to_vec().unwrap());

    let destination = Pubkey::new_from_array(
        Vec::from(&message_payload[..32])
            .as_slice()
            .try_into()
            .unwrap(),
    );

    let keys = accounts
        .into_iter()
        .map(|k| *k.key)
        .collect::<Vec<Pubkey>>();

    msg!("KEYS: {:?}", keys);

    invoke_execute(
        &destination,
        message_data,
        sysvar_instructions,
        accounts_iter.as_slice(),
        Vec::from(&message_payload[32..]),
    )?;

    Ok(())
}
