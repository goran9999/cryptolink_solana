use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};

pub fn check_seeds(
    account: &Pubkey,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, ProgramError> {
    let (target_account, bump) = Pubkey::find_program_address(seeds, program_id);

    if account != &target_account {
        return Err(ProgramError::InvalidSeeds);
    }

    Ok(bump)
}

pub fn check_account_signer(signer: &AccountInfo) -> ProgramResult {
    if !signer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn transfer_sol<'a, 'b>(
    source: &'a AccountInfo<'b>,
    destination: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    lamports: u64,
) -> ProgramResult {
    let instruction = system_instruction::transfer(source.key, destination.key, lamports);

    invoke(
        &instruction,
        &[source.clone(), destination.clone(), system_program.clone()],
    )?;

    Ok(())
}

pub fn create_account<'a, 'b>(
    from: &'a AccountInfo<'b>,
    to: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    space: u64,
    owner: &Pubkey,
    seeds: &[&[u8]],
) -> ProgramResult {
    let rent = Rent::default().minimum_balance(space as usize);

    let ix = system_instruction::create_account(from.key, to.key, rent, space, owner);

    invoke_signed(
        &ix,
        &[from.clone(), to.clone(), system_program.clone()],
        &[seeds],
    )?;

    Ok(())
}
