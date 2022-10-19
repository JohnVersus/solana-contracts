use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(hello_world);

pub fn hello_world(
    program_id: &Pubkey,      // Public key of the account the program was loaded into
    accounts: &[AccountInfo], // All accounts required to process the instruction
    instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult {
    msg!("Hello {:}!!", accounts[0].key);
    Ok(())
}
