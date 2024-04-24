use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::state::ForeignAddress;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Mv3SenderInstruction {
    SendData {
        destination_chain_id: u64,
        payload: Vec<u8>,
        destination: ForeignAddress,
    },
}

pub fn get_message_pda(sender: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    let pda = Pubkey::find_program_address(&[MESSENGER_SEED, sender.as_ref()], program_id);

    pda
}

pub const MESSENGER_SEED: &[u8] = b"messenger";
pub const CONFIG_SEED: &[u8] = b"config";

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessagePayload {
    pub destination_chain_id: u64,
    pub payload: Vec<u8>,
    pub destination: ForeignAddress,
}

pub fn send_message(
    program_id: Pubkey,
    payer: &Pubkey,
    sender_program_id: &Pubkey,
    destination_chain_id: u64,
    destination: ForeignAddress,
    payload: Vec<u8>,
) -> Instruction {
    let (pda, _) = get_message_pda(sender_program_id, &program_id);

    let (config_pda, _) = get_config_pda();

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: config_pda,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: pda,
        },
        AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: *sender_program_id,
        },
        AccountMeta {
            is_signer: true,
            is_writable: true,
            pubkey: *payer,
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: sysvar::instructions::id(),
        },
        AccountMeta {
            is_signer: false,
            is_writable: false,
            pubkey: system_program::id(),
        },
    ];

    let data = Mv3SenderInstruction::SendData {
        destination_chain_id,
        payload,
        destination,
    }
    .try_to_vec()
    .unwrap();

    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn get_config_pda() -> (Pubkey, u8) {
    let pda = Pubkey::find_program_address(&[CONFIG_SEED], &crate::id());

    pda
}
