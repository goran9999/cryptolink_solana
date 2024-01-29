use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
    secp256k1_recover::secp256k1_recover,
};

use crate::{
    constants::SOLANA_CHAIN_ID,
    error::MessengerError,
    instruction::ReceiveMessage,
    state::config::{MessengerConfig, Role},
    utils::{assert_account_signer, role_guard},
};

pub fn process_receive_message(
    receive_message: ReceiveMessage,
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let signer = next_account_info(accounts_iter)?;

    assert_account_signer(signer)?;

    let raw_config = next_account_info(accounts_iter)?;

    let config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow())?;

    //only operator can sign message processing
    role_guard(&config, signer, Role::Operator)?;

    if !config.bridge_enabled {
        return Err(MessengerError::BrigdeNotEnabled.into());
    }

    if receive_message.dest_chain_id != SOLANA_CHAIN_ID {
        return Err(MessengerError::ChainNotSupported.into());
    }

    let src_chain_exists = config
        .enabled_chains
        .into_iter()
        .any(|c| c == receive_message.source_chain_id);

    if !src_chain_exists {
        return Err(MessengerError::ChainNotSupported.into());
    }

    //TODO:find best way to store processed txs

    if let Some(exsig) = config
        .exsig
        .iter()
        .find(|exsig| exsig.recipient == receive_message.receiver)
    {
        let exsig_vrs_bytes = receive_message
            .data
            .get(0)
            .expect("Missing Exsig vrs bytes!");

        if exsig_vrs_bytes.len() != 65 {
            return Err(MessengerError::InvalidSignature.into());
        }

        let recovered_signer = secp256k1_recover(
            &exsig_vrs_bytes[65..],
            exsig_vrs_bytes[0],
            &exsig_vrs_bytes[1..65],
        )
        .expect("Failed to recover secp256k1 sig");

        let signer = recovered_signer.0.try_to_vec().unwrap();

        if signer.as_slice() != exsig.sig {
            return Err(MessengerError::InvalidSignature.into());
        }
    }

    if let Some(chainsig) = config.chainsig {
        let chainsig_vrs_bytes = receive_message
            .data
            .get(1)
            .expect("Missing Chainsig vrs bytes!");

        if chainsig_vrs_bytes.len() != 65 {
            return Err(MessengerError::InvalidSignature.into());
        }

        let recovered_chainsig = secp256k1_recover(
            &chainsig_vrs_bytes[65..],
            chainsig_vrs_bytes[0],
            &chainsig_vrs_bytes[1..65],
        )
        .expect("Failed to recover secp256k1 sig");

        msg!("CHAINSIG: {:?}", recovered_chainsig.0);

        let chainsig_address = recovered_chainsig.0;

        if chainsig_address[12..] != chainsig[12..] {
            return Err(MessengerError::InvalidSignature.into());
        }
    }

    let cpi_accounts: Vec<AccountMeta> = accounts[2..]
        .iter()
        .map(|acc| AccountMeta {
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
            pubkey: acc.key.clone(),
        })
        .collect();

    let cpi_data = receive_message
        .data
        .get(2)
        .expect("Missing target program data");

    let ix: Instruction = Instruction {
        program_id: receive_message.receiver,
        accounts: cpi_accounts,
        data: cpi_data.clone(),
    };

    let _result = invoke(&ix, &accounts[2..]);

    //TODO: store failed and succeeded tx
    // match result {
    //     Ok(()) => {}
    //     Err(err) => {}
    // }

    Ok(())
}
