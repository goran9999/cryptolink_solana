use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{self, create_account},
};

use crate::{
    error::MessengerError,
    state::config::{MessengerConfig, Role},
};

pub fn initialize_account<'a, 'b>(
    from: &'a AccountInfo<'b>,
    account: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    space: u64,
    owner_program: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    let rent = Rent::default().minimum_balance(space.try_into().unwrap());

    let create_account_ix = create_account(from.key, account.key, rent, space, owner_program);

    invoke_signed(
        &create_account_ix,
        &[
            account.to_owned(),
            from.to_owned(),
            system_program.to_owned(),
        ],
        &[seeds],
    )?;

    Ok(())
}

pub fn check_keys_eq(account_1: &Pubkey, account_2: &Pubkey) -> Result<(), ProgramError> {
    if account_1 != account_2 {
        return Err(MessengerError::PublicKeyMissmatch.into());
    }

    Ok(())
}

pub fn role_guard(
    config: &MessengerConfig,
    checked_account: &AccountInfo,
    role: Role,
) -> Result<(), ProgramError> {
    match role {
        Role::ATeam => {
            if config
                .bridge_a_team
                .iter()
                .any(|a_team| a_team.wallet == *checked_account.key && a_team.is_active)
            {
                return Ok(());
            }
            return Err(MessengerError::CallerNotATeam.into());
        }
        Role::Super => {
            if config.bridge_supers.iter().any(|super_account| {
                super_account.wallet == *checked_account.key && super_account.is_active
            }) {
                return Ok(());
            }
            return Err(MessengerError::CallerNotSuper.into());
        }
        Role::Operator => {
            if config
                .bridge_operators
                .iter()
                .any(|operator| operator.wallet == *checked_account.key && operator.is_active)
            {
                return Ok(());
            }
            return Err(MessengerError::CallerNotOperator.into());
        }
        _ => {
            return Ok(());
        }
    }
}

pub fn check_target_chain(config: MessengerConfig, target_chain: &u32) -> Result<(), ProgramError> {
    if !config
        .enabled_chains
        .iter()
        .any(|chain| chain == target_chain)
    {
        return Err(MessengerError::ChainNotSupported.into());
    }

    Ok(())
}

pub fn get_next_tx_id(config: &MessengerConfig) -> u128 {
    let next_tx_id = config.next_tx_id.checked_add(1).expect("Oveflow");

    next_tx_id
}

pub fn check_seeds(
    account: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, ProgramError> {
    let (target_key, bump) = Pubkey::find_program_address(seeds, program_id);

    if *account.key != target_key {
        return Err(MessengerError::InvalidAccountSeeds.into());
    }

    Ok(bump)
}

pub fn assert_account_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer {
        return Err(MessengerError::AccountNotSigner.into());
    }

    Ok(())
}

pub fn transfer_sol<'a, 'b>(
    from: &'a AccountInfo<'b>,
    to: &'a AccountInfo<'b>,
    lamports: u64,
    system_program: &'a AccountInfo<'b>,
    seeds: Option<&[&[u8]]>,
) -> Result<(), ProgramError> {
    let ix = system_instruction::transfer(from.key, to.key, lamports);

    let accounts: &[AccountInfo] = &[from.clone(), to.clone(), system_program.clone()];

    if let Some(signer_seeds) = seeds {
        invoke_signed(&ix, accounts, &[signer_seeds])?;
    } else {
        invoke(&ix, accounts)?;
    }

    Ok(())
}

pub fn check_secp256k1_data(
    data: &[u8],
    eth_address: &[u8],
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> ProgramResult {
    // According to this layout used by the Secp256k1Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/secp256k1-program.ts#L49

    // "Deserializing" byte slices

    let num_signatures = &[data[0]]; // Byte  0
    let signature_offset = &data[1..=2]; // Bytes 1,2
    let signature_instruction_index = &[data[3]]; // Byte  3
    let eth_address_offset = &data[4..=5]; // Bytes 4,5
    let eth_address_instruction_index = &[data[6]]; // Byte  6
    let message_data_offset = &data[7..=8]; // Bytes 7,8
    let message_data_size = &data[9..=10]; // Bytes 9,10
    let message_instruction_index = &[data[11]]; // Byte  11

    let data_eth_address = &data[12..12 + 20]; // Bytes 12..12+20
    let data_sig = &data[32..32 + 64]; // Bytes 32..32+64
    let data_recovery_id = &[data[96]]; // Byte  96
    let data_msg = &data[97..]; // Bytes 97..end

    // Expected values

    const SIGNATURE_OFFSETS_SERIALIZED_SIZE: u16 = 11;
    const DATA_START: u16 = 1 + SIGNATURE_OFFSETS_SERIALIZED_SIZE;

    let msg_len: u16 = msg.len().try_into().unwrap();
    let eth_address_len: u16 = eth_address.len().try_into().unwrap();
    let sig_len: u16 = sig.len().try_into().unwrap();

    let exp_eth_address_offset: u16 = DATA_START;
    let exp_signature_offset: u16 = DATA_START + eth_address_len;
    let exp_message_data_offset: u16 = exp_signature_offset + sig_len + 1;
    let exp_num_signatures: u8 = 1;

    // Header and Arg Checks

    msg!("DATA ETH ADDR: {:?}", data_eth_address);

    // Header
    if num_signatures != &exp_num_signatures.to_le_bytes()
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &[0]
        || eth_address_offset != &exp_eth_address_offset.to_le_bytes()
        || eth_address_instruction_index != &[0]
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &msg_len.to_le_bytes()
        || message_instruction_index != &[0]
    {
        return Err(MessengerError::InvalidSignature.into());
    }

    // Arguments
    if data_eth_address != eth_address
        || data_sig != sig
        || data_recovery_id != &[recovery_id]
        || data_msg != msg
    {
        return Err(MessengerError::InvalidSignature.into());
    }

    Ok(())
}
