use solana_program::{
    account_info::AccountInfo, borsh0_10::try_from_slice_unchecked, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

mod process_add_user_permission;
mod process_change_config;
mod process_configure_client;
mod process_deposit_withdraw;
mod process_initialize_config;
mod process_receive_message;
mod process_send_message;
mod process_set_exsig;

use crate::{
    instruction::{
        AddUserPermission, ChangeConfig, InitializeConfig, ReceiveMessage, SendMessage, SetExsig,
        V3Instruction,
    },
    state::config::MessageClient,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Deserializing ix! {}", data.len());
    let instruction = try_from_slice_unchecked::<V3Instruction>(data)?;

    match instruction {
        V3Instruction::InitializeConfig { accountant } => {
            msg!("MessageV3: Initialize Config!");
            process_initialize_config::process_initialize_config(
                accounts,
                program_id,
                InitializeConfig { accountant },
            )?;
        }
        V3Instruction::AddUserPermission {
            user,
            is_active,
            role,
        } => {
            msg!("MessageV3: Adding user permissions!");
            process_add_user_permission::process_add_user_permission(
                accounts,
                program_id,
                AddUserPermission {
                    role,
                    user,
                    is_active,
                },
            )?
        }
        V3Instruction::ChangeConfig {
            enabled_chains,
            bridge_enabled,
            accountant,
            whitelist_only,
            chainsig,
        } => {
            msg!("MessageV3: Modify Config!");

            process_change_config::process_change_config(
                accounts,
                program_id,
                ChangeConfig {
                    accountant,
                    whitelist_only,
                    chainsig,
                    enabled_chains,
                    bridge_enabled,
                },
            )?
        }
        V3Instruction::Send {
            recipient,
            chain,
            confirmations,
            data,
        } => process_send_message::process_send_message(
            program_id,
            accounts,
            SendMessage {
                recipient,
                chain,
                data,
                confirmations,
            },
        )?,
        V3Instruction::SetExsig { exsig } => {
            process_set_exsig::process_set_exsig(program_id, accounts, SetExsig { exsig })?
        }
        V3Instruction::ConfigureClient {
            authority,
            destination_contract,
            notify_on_failure,
            supported_chains,
            allowed_contracts,
            exsig,
        } => {
            process_configure_client::process_configure_client(
                program_id,
                accounts,
                MessageClient {
                    allowed_contracts,
                    authority,
                    destination_contract,
                    exsig,
                    notify_on_failure,
                    supported_chains,
                },
            )?;
        }
        V3Instruction::ReceiveMessage {
            tx_id,
            dest_chain_id,
            receiver,
            data,
            source_chain_id,
            sender,
        } => {
            msg!("MessageV3: Receive Message!");

            process_receive_message::process_receive_message(
                ReceiveMessage {
                    tx_id,
                    dest_chain_id,
                    receiver,
                    data,
                    source_chain_id,
                    sender,
                },
                program_id,
                accounts,
            )?
        }
        V3Instruction::DepositWithdraw { action, amount } => {
            process_deposit_withdraw::process_deposit_withdraw(
                program_id, accounts, amount, action,
            )?;
        }
    }

    Ok(())
}
