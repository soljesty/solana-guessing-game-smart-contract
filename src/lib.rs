use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,     // Program ID is not used in this example
    accounts: &[AccountInfo], // Accounts involved in the transaction
    _instruction_data: &[u8], // Instruction data (not used)
) -> ProgramResult {
    if accounts.len() < 2 {
        msg!("Error: Not enough accounts provided.");
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let payer = &accounts[0];     // User's wallet
    let receiver = &accounts[1];  // Game's account

    let amount = 1_000_000; // 0.001 SOL in lamports (1 SOL = 1,000,000,000 lamports)

    msg!("Attempting to transfer 0.001 SOL from {} to {}", payer.key, receiver.key);

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
