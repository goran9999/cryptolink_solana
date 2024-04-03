use message_hook::instruction::MessageHookInstruction;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

mod process_hop;
mod process_initialize_extra_account_meta_list;
mod process_update_extra_account_meta_list;
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let unpacked = MessageHookInstruction::unpack(data);

    if let Ok(message_hook_instruction) = unpacked {
        match message_hook_instruction {
            MessageHookInstruction::InitializeExtraAccountMetaList {
                extra_account_metas,
            } => {
                msg!("IX: Init extra account meta list!");

                process_initialize_extra_account_meta_list::process_initialize_extra_account_meta_list(program_id, accounts, &extra_account_metas)?
            }
            MessageHookInstruction::ProcessMessage { data } => {
                msg!("IX: Process message");

                process_hop::process_hop(program_id, accounts, data)?;
            }
            MessageHookInstruction::UpdateExtraAccountMetaList {
                extra_account_metas,
            } => process_update_extra_account_meta_list::process_update_extra_account_meta_list(
                program_id,
                accounts,
                &extra_account_metas,
            )?,
        }

        return Ok(());
    }

    Ok(())
}
