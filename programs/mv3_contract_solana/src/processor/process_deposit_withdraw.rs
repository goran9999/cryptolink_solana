use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

use crate::{
    constants::{MESSAGE_CLIENT_SEED, MESSAGE_CLIENT_TREASURY_SEED},
    error::MessengerError,
    instruction::DepositWithdraw,
    state::config::MessageClient,
    utils::check_client_treasury_seeds,
};

pub fn process_deposit_withdraw(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    action: DepositWithdraw,
) -> ProgramResult {
    let accounts = &mut accounts.iter();
    let payer = next_account_info(accounts)?;
    let message_client = next_account_info(accounts)?;
    let treasury = next_account_info(accounts)?;
    let system_program = next_account_info(accounts)?;

    let decoded_client = try_from_slice_unchecked::<MessageClient>(&message_client.data.borrow())?;

    let bump = check_client_treasury_seeds(decoded_client.destination_contract, *treasury.key)?;

    if decoded_client.authority != *payer.key {
        return Err(MessengerError::InvalidUpdateAuthority.into());
    }

    match action {
        DepositWithdraw::Deposit => {
            let ix = system_instruction::transfer(payer.key, treasury.key, amount);

            invoke(
                &ix,
                &[
                    payer.to_owned(),
                    treasury.to_owned(),
                    system_program.to_owned(),
                ],
            )?;
        }
        DepositWithdraw::Withdraw => {
            let ix = system_instruction::transfer(treasury.key, payer.key, amount);
            invoke_signed(
                &ix,
                &[
                    treasury.to_owned(),
                    payer.to_owned(),
                    system_program.to_owned(),
                ],
                &[&[
                    MESSAGE_CLIENT_SEED,
                    decoded_client.destination_contract.as_ref(),
                    MESSAGE_CLIENT_TREASURY_SEED,
                    &[bump],
                ]],
            )?;
        }
    }

    Ok(())
}
