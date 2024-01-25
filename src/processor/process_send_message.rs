use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    system_program::ID,
    sysvar::instructions::{
        load_current_index_checked, load_instruction_at_checked, ID as SysvarInstructions,
    },
};

use crate::{
    constants::{CALLER_INSTRUCTION_DISCRIMINATOR, CALLER_PROGRAM, MESSAGE_SEED, MESSENGER_SEED},
    error::MessengerError,
    instruction::SendMessage,
    state::{
        config::{ForeignAddress, MessengerConfig, Role},
        message::Message,
    },
    utils::{
        check_keys_eq, check_seeds, get_next_tx_id, initialize_account, role_guard, transfer_sol,
    },
};

pub fn process_send_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: SendMessage,
) -> ProgramResult {
    let accounts = &mut accounts.iter();

    let raw_config = next_account_info(accounts)?;

    check_seeds(raw_config, &[MESSENGER_SEED], program_id)?;

    let sender = next_account_info(accounts)?;

    let raw_message = next_account_info(accounts)?;

    let system_program = next_account_info(accounts)?;

    let sysvar_instructions = next_account_info(accounts)?;

    check_keys_eq(sysvar_instructions.key, &SysvarInstructions)?;

    check_keys_eq(system_program.key, &ID)?;

    let current_ix_index = load_current_index_checked(sysvar_instructions)?;

    let previous_ix =
        load_instruction_at_checked(usize::from(current_ix_index - 1), sysvar_instructions)
            .map_err(|_| MessengerError::InvalidInstructionIndex)
            .unwrap();

    if previous_ix.program_id != CALLER_PROGRAM.parse().unwrap() {
        return Err(MessengerError::InvalidPreInstruction.into());
    }

    let previous_ix_data = &previous_ix.data[0];

    if previous_ix_data != &CALLER_INSTRUCTION_DISCRIMINATOR {
        return Err(MessengerError::InvalidPreInstruction.into());
    }

    let total_len = Message::LEN + data.data.len();
    if raw_message.data_is_empty() {
        let lamports = Rent::default().minimum_balance(total_len);

        transfer_sol(sender, raw_message, lamports, system_program, None)?;
        initialize_account(
            sender,
            raw_message,
            system_program,
            total_len as u64,
            program_id,
            &[MESSAGE_SEED],
        )?;
    } else {
        let additional_length = raw_config.data_len().checked_sub(total_len).unwrap_or(0);

        if additional_length > 0 {
            let realloc_fee = Rent::default().minimum_balance(additional_length);

            transfer_sol(sender, raw_message, realloc_fee, system_program, None)?;

            raw_message.realloc(total_len.checked_add(additional_length).unwrap(), false)?;
        }
    }

    let mut config: MessengerConfig = try_from_slice_unchecked(&raw_config.data.borrow())?;

    if !config.bridge_enabled {
        return Err(MessengerError::BrigdeNotEnabled.into());
    }

    let chain_exist = config.enabled_chains.iter().any(|c| *c == data.chain);

    if !chain_exist {
        return Err(MessengerError::ChainNotSupported.into());
    }

    if config.whitelist_only {
        role_guard(&config, sender, Role::Whitelist)?;
    }

    let next_tx_id = get_next_tx_id(&config);

    config.next_tx_id = next_tx_id;

    raw_message.data.borrow_mut().copy_from_slice(
        &Message {
            chain: data.chain,
            confirmations: data.confirmations,
            data: data.data,
            recipient: data.recipient,
            sender: *sender.key,
            tx_id: next_tx_id,
        }
        .try_to_vec()
        .unwrap(),
    );
    raw_config
        .data
        .borrow_mut()
        .serialize(&mut config.try_to_vec().unwrap())?;

    Ok(())
}
