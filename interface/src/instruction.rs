use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};
use spl_pod::{bytemuck::pod_slice_to_bytes, slice::PodSlice};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum MessageHookInstruction {
    ProcessMessage {
        data: Vec<u8>,
    },

    InitializeExtraAccountMetaList {
        extra_account_metas: Vec<ExtraAccountMeta>,
    },

    UpdateExtraAccountMetaList {
        extra_account_metas: Vec<ExtraAccountMeta>,
    },
}

#[derive(SplDiscriminate)]
#[discriminator_hash_input("message-hook-interface:process-message")]
pub struct ProcessMessageInstruction;

#[derive(SplDiscriminate)]
#[discriminator_hash_input("message-hook-interface:initialize-extra-account-metas")]
pub struct InitializeExtraAccountMetaListInstruction;

#[derive(SplDiscriminate)]
#[discriminator_hash_input("message-hook-interface:update-extra-account-metas")]
pub struct UpdateExtraAccountMetaListInstruction;

impl MessageHookInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (discriminator, rest) = data.split_at(ArrayDiscriminator::LENGTH);

        Ok(match discriminator {
            ProcessMessageInstruction::SPL_DISCRIMINATOR_SLICE => Self::ProcessMessage {
                data: Vec::from(rest),
            },
            InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE => {
                let pod_slice = PodSlice::<ExtraAccountMeta>::unpack(rest)?;
                let extra_account_metas = pod_slice.data().to_vec();
                Self::InitializeExtraAccountMetaList {
                    extra_account_metas,
                }
            }
            UpdateExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE => {
                let pod_slice = PodSlice::<ExtraAccountMeta>::unpack(rest)?;
                let extra_account_metas = pod_slice.data().to_vec();
                Self::UpdateExtraAccountMetaList {
                    extra_account_metas,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];

        match self {
            Self::ProcessMessage { data } => {
                buf.extend_from_slice(ProcessMessageInstruction::SPL_DISCRIMINATOR_SLICE);
                buf.extend_from_slice(data);
            }
            Self::InitializeExtraAccountMetaList {
                extra_account_metas,
            } => {
                buf.extend_from_slice(
                    InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE,
                );
                buf.extend_from_slice(&(extra_account_metas.len() as u32).to_le_bytes());
                buf.extend_from_slice(pod_slice_to_bytes(extra_account_metas));
            }
            Self::UpdateExtraAccountMetaList {
                extra_account_metas,
            } => {
                buf.extend_from_slice(
                    UpdateExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE,
                );
                buf.extend_from_slice(&(extra_account_metas.len() as u32).to_le_bytes());
                buf.extend_from_slice(pod_slice_to_bytes(extra_account_metas));
            }
        }

        buf
    }
}

pub fn execute(
    program_id: &Pubkey,
    message: &Pubkey,
    validate_state_pubkey: &Pubkey,
    data: Vec<u8>,
) -> Instruction {
    let data = MessageHookInstruction::ProcessMessage { data }.pack();

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new_readonly(*message, false),
        AccountMeta::new_readonly(sysvar::instructions::id(), false),
        AccountMeta::new_readonly(*validate_state_pubkey, false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

pub fn initialize_extra_account_meta_list(
    program_id: &Pubkey,
    extra_account_metas_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    extra_account_metas: &[ExtraAccountMeta],
) -> Instruction {
    let data = MessageHookInstruction::InitializeExtraAccountMetaList {
        extra_account_metas: extra_account_metas.to_vec(),
    }
    .pack();

    let accounts = vec![
        AccountMeta::new(*extra_account_metas_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*authority_pubkey, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

pub fn update_extra_account_meta_list(
    program_id: &Pubkey,
    extra_account_metas_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    extra_account_metas: &[ExtraAccountMeta],
) -> Instruction {
    let data = MessageHookInstruction::UpdateExtraAccountMetaList {
        extra_account_metas: extra_account_metas.to_vec(),
    }
    .pack();

    let accounts = vec![
        AccountMeta::new(*extra_account_metas_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*authority_pubkey, true),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
