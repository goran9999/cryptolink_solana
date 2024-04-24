use std::ops::Div;

use borsh::BorshSerialize;
use ethnum::U256;
use message_hook::instruction::ProcessMessageInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

use crate::instructions::{HopData, EVM_HOP_CONTRACT};

pub fn process_hop(program_id: &Pubkey, accounts: &[AccountInfo], data: Vec<u8>) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;

    let message = next_account_info(accounts_iter)?;

    let source = U256::from_be_bytes(data[0..32].try_into().unwrap());

    msg!("Source: {:?}", source);

    let hop = U256::from_be_bytes(data[32..64].try_into().unwrap()).as_u64();

    let remainin_len = data[128..].len().div(32);

    let mut chainlist: Vec<u64> = vec![];

    for i in 0..remainin_len {
        let bytes = i * 32;

        let chain = U256::from_be_bytes(data[128 + bytes..128 + bytes + 32].try_into().unwrap());

        msg!("Chain: {:?}", chain);

        chainlist.push(chain.as_u64());
    }

    let sysvar_instructions = next_account_info(accounts_iter)?;

    if hop >= (chainlist.len() as u64) {
        msg!("Hop completed!");

        return Ok(());
    }

    let next_hop = usize::try_from(hop + 1).unwrap();

    let next_hop: &u64 = chainlist.get(next_hop).unwrap();

    //we do reverse because solana is little-endian encoded while EVM is big-endian

    let mut chainlist_serialized = chainlist.try_to_vec().unwrap();
    chainlist_serialized.reverse();

    let mut next_hop_serialized = next_hop.try_to_vec().unwrap();
    next_hop_serialized.reverse();

    let mut chain_serialized = (19999999991_u64).try_to_vec().unwrap();
    chain_serialized.reverse();

    let payload = HopData {
        chainlist: chainlist_serialized,
        hop: next_hop_serialized,
        source_chain: chain_serialized,
    }
    .try_to_vec()
    .unwrap();

    let extra_account_meta = next_account_info(accounts_iter)?;

    ExtraAccountMetaList::check_account_infos::<ProcessMessageInstruction>(
        accounts,
        data.as_ref(),
        program_id,
        &extra_account_meta.data.borrow(),
    )?;

    let ix = mv3_solana_sender::instructions::send_message(
        mv3_solana_sender::id(),
        payer.key,
        program_id,
        *next_hop,
        EVM_HOP_CONTRACT,
        payload,
    );

    invoke(
        &ix,
        &[
            payer.to_owned(),
            message.to_owned(),
            sysvar_instructions.to_owned(),
        ],
    )?;

    Ok(())
}
