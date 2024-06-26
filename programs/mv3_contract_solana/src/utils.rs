use sha3::{Digest, Keccak256};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{self, create_account},
};
use tiny_keccak::{Hasher, Keccak};

use crate::{
    constants::{
        GLOBAL_TREASURY, MESSAGE_CLIENT_SEED, MESSAGE_CLIENT_TREASURY_SEED, MESSAGE_SEED, PREFIX,
    },
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

pub fn check_client_seeds(
    destination_contract: Pubkey,
    address: Pubkey,
) -> Result<u8, ProgramError> {
    let (pda, bump) = get_message_client_pda(destination_contract);

    if address != pda {
        return Err(ProgramError::InvalidSeeds.into());
    }

    Ok(bump)
}

pub fn check_client_treasury_seeds(
    destination_contract: Pubkey,
    address: Pubkey,
) -> Result<u8, ProgramError> {
    let (pda, bump) = Pubkey::find_program_address(
        &[
            MESSAGE_CLIENT_SEED,
            destination_contract.as_ref(),
            MESSAGE_CLIENT_TREASURY_SEED,
        ],
        &crate::id(),
    );

    if pda != address {
        return Err(ProgramError::InvalidSeeds.into());
    }

    Ok(bump)
}

pub fn get_client_treasury_pda(destination_contract: Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[
            MESSAGE_CLIENT_SEED,
            destination_contract.as_ref(),
            MESSAGE_CLIENT_TREASURY_SEED,
        ],
        &crate::id(),
    );

    pda
}

pub fn get_global_treasury_pda() -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(&[MESSAGE_SEED, GLOBAL_TREASURY], &crate::id());
    pda
}

pub fn check_global_treasury_seeds(address: Pubkey) -> Result<u8, ProgramError> {
    let (pda, bump) = Pubkey::find_program_address(&[MESSAGE_SEED, GLOBAL_TREASURY], &crate::id());

    if pda != address {
        return Err(ProgramError::InvalidSeeds.into());
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

pub fn get_message_pda(program_id: &Pubkey) -> Pubkey {
    let (message_key, _) =
        Pubkey::find_program_address(&[MESSAGE_SEED, program_id.as_ref()], &crate::id());

    message_key
}

pub fn public_key_to_address(pub_key: &[u8]) -> [u8; 20] {
    let mut hasher = Keccak256::new();

    if pub_key[0] == 4 {
        hasher.update(&pub_key[1..]);
    } else {
        hasher.update(pub_key);
    }

    let result = hasher.finalize();

    result[12..32].try_into().unwrap()
}

pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

pub fn pubkey_to_address(evm_pubkey: &[u8]) -> String {
    evm_pubkey
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect()
}

pub fn create_ecdsa_sig(message: &Vec<u8>) -> [u8; 32] {
    let mut eth_message = format!("{}{}", PREFIX, message.len()).into_bytes();
    eth_message.extend_from_slice(&message);

    let hashed = keccak256(&eth_message);

    hashed
}

pub fn get_message_client_pda(destination_contract: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[MESSAGE_CLIENT_SEED, destination_contract.as_ref()],
        &crate::id(),
    )
}
