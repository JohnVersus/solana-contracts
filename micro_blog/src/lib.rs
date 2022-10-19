use borsh::{BorshDeserialize, BorshSerialize};
use std::str;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// Create a struct to store Blog count
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BlogCount {
    pub total_blogs: u32,
}

// Function to convert buffer array back to string
pub fn buffer_to_string(buffer: &[u8]) -> &str {
    let s = match str::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    return s;
}

entrypoint!(micro_blog);

pub fn micro_blog(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let data = buffer_to_string(&instruction_data);

    let account = &accounts[0];

    // Check if the account is owned by this program, else throw an error.
    if account.owner != program_id {
        msg!(
            "Account {:?} does not have the program id {} as owner",
            account,
            program_id
        );
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times user created a new blog.
    let mut blog_counter = BlogCount::try_from_slice(&account.data.borrow())?;
    blog_counter.total_blogs += 1;
    blog_counter.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // Save the data to the transaction logs
    msg!("Author: {}", accounts[1].key);
    msg!("Blog No: {}", blog_counter.total_blogs);
    msg!("Blog: {}", data);

    Ok(())
}
