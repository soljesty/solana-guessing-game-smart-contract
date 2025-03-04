use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};
use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ensure there are at least two accounts: payer and receiver
    if accounts.len() < 2 {
        msg!("Error: Not enough accounts provided.");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let payer = &accounts[0];     // User's wallet (payer)
    let receiver = &accounts[1];  // Receiver's wallet

    // Ensure the payer is the signer
    if !payer.is_signer {
        msg!("Error: Payer is not the signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check ownership of payer and receiver accounts
    if *payer.owner != system_program::ID {
        msg!("Error: Payer is not owned by the system program.");
        return Err(ProgramError::IncorrectProgramId);
    }

    if *receiver.owner != system_program::ID {
        msg!("Error: Receiver is not owned by the system program.");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse the transfer amount from instruction data
    if instruction_data.len() < 8 {
        msg!("Error: Instruction data is too short.");
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Ensure the transfer amount is greater than zero
    if amount == 0 {
        msg!("Error: Transfer amount must be greater than zero.");
        return Err(ProgramError::InvalidInstructionData);
    }

    // Ensure the payer has enough funds to make the transfer
    if **payer.lamports.borrow() < amount {
        msg!("Error: Payer has insufficient funds.");
        return Err(ProgramError::InsufficientFunds);
    }

    msg!("Attempting to transfer {} lamports...", amount);

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        payer.key,
        receiver.key,
        amount,
    );

    // Invoke the transfer instruction
    invoke(
        &transfer_instruction,
        &[payer.clone(), receiver.clone()],
    )?;

    msg!("Transfer successful.");

    Ok(())
}

