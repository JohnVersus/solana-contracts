use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

pub fn win_check(moves: [u32; 9]) -> u32 {
    // Player 1 move will be marked as 1 and player 2 as 2
    let [m1, m2, m3, m4, m5, m6, m7, m8, m9] = moves;
    if (m1 == 1 && m2 == 1 && m3 == 1)
        || (m1 == 1 && m4 == 1 && m7 == 1)
        || (m7 == 1 && m8 == 1 && m9 == 1)
        || (m3 == 1 && m6 == 1 && m9 == 1)
        || (m1 == 1 && m5 == 1 && m9 == 1)
        || (m3 == 1 && m5 == 1 && m7 == 1)
        || (m2 == 1 && m5 == 1 && m8 == 1)
        || (m4 == 1 && m5 == 1 && m6 == 1)
    {
        // Condition for Player 1 Win
        return 1;
    } else if (m1 == 2 && m2 == 2 && m3 == 2)
        || (m1 == 2 && m4 == 2 && m7 == 2)
        || (m7 == 2 && m8 == 2 && m9 == 2)
        || (m3 == 2 && m6 == 2 && m9 == 2)
        || (m1 == 2 && m5 == 2 && m9 == 2)
        || (m3 == 2 && m5 == 2 && m7 == 2)
        || (m2 == 2 && m5 == 2 && m8 == 2)
        || (m4 == 2 && m5 == 2 && m6 == 2)
    {
        // Condition for Player 2 Win
        return 2;
    } else if (m1 == 1 || m1 == 2)
        && (m2 == 1 || m2 == 2)
        && (m3 == 1 || m3 == 2)
        && (m4 == 1 || m4 == 2)
        && (m5 == 1 || m5 == 2)
        && (m6 == 1 || m6 == 2)
        && (m7 == 1 || m7 == 2)
        && (m8 == 1 || m8 == 2)
        && (m9 == 1 || m9 == 2)
    {
        // Condition for Draw
        return 3;
    } else {
        return 0;
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameAccount {
    pub player1: String,
    pub player2: String,
    pub moves: [u32; 9],
    pub game_status: u32,
    pub next_move: u32,
}

entrypoint!(tic_tac_toe);

pub fn tic_tac_toe(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let game_account = &accounts[0];
    let player1 = accounts[1].key.to_string();
    let player2 = accounts[2].key.to_string();

    let instruction: u32 = instruction_data[0].into();
    let played_by: u32 = instruction_data[1].into();
    let move_positon: usize = instruction_data[2].into();

    match instruction {
        // Create New Game or Reset the Game Data
        0 => {
            msg!("Instruction 0 Start");
            let game_data = GameAccount {
                player1,
                player2,
                moves: [0, 0, 0, 0, 0, 0, 0, 0, 0],
                game_status: 0,
                next_move: 1,
            };
            msg!("Game Creation Successful!!");
            msg!("Player 1: {:?}", game_data.player1);
            msg!("Player 2: {:?}", game_data.player2);
            game_data.serialize(&mut &mut game_account.data.borrow_mut()[..])?;
            msg!("Instruction 0 End");
        }
        // Play game!!
        1 => {
            msg!("Instruction 1 Start");
            let mut game_data = GameAccount::try_from_slice(&game_account.data.borrow())?;
            if game_data.game_status == 0 {
                msg!("Player 1: {:?}", game_data.player1);
                msg!("Player 2: {:?}", game_data.player2);

                // Verify and updating moves in Game Account
                if (game_data.moves[move_positon] == 0) && (game_data.next_move == played_by) {
                    if game_data.next_move == 1 {
                        game_data.moves[move_positon] = 1;
                        game_data.next_move = 2
                    } else if game_data.next_move == 2 {
                        game_data.moves[move_positon] = 2;
                        game_data.next_move = 1
                    }
                } else {
                    msg!(" Wrong Move");
                }

                let game_status = win_check(game_data.moves);

                match game_status {
                    0 => {
                        // Log the next player to move
                        msg!("Next move: Player {}", game_data.next_move);
                    }
                    1 => {
                        game_data.game_status = 1;
                        msg!("Player 1 won the game.");
                    }
                    2 => {
                        game_data.game_status = 2;
                        msg!("Player 2 won the game.");
                    }
                    3 => {
                        game_data.game_status = 3;
                        msg!("It's a Draw.");
                    }
                    _ => {
                        msg!("Game Error!!");
                    }
                }
                // Write the updated data to account.
                game_data.serialize(&mut &mut game_account.data.borrow_mut()[..])?;
                msg!("Instruction 1 End");
            } else {
                msg!(" Wrong Move.");
            }
        }
        // Invalid Instruction
        _ => {
            msg!("Invalid Instruction");
        }
    }

    Ok(())
}
