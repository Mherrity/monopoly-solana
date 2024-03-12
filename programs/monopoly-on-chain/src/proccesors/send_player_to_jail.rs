use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn send_player_to_jail(ctx: Context<FinishTurn>) -> Result<()>{
    let player_state = &mut ctx.accounts.player_state;
    require!(player_state.position == GO_TO_JAIL_POSITION, ProgramError::NotOnGoToJailPosition);
    player_state.position = JAIL_POSITION; 
    Ok(())
}


const JAIL_POSITION: u8 = 10; // Jail position, adjust as per your game board
const GO_TO_JAIL_POSITION: u8 = 7; // Go to jail position, adjust as per your game board

#[error_code]
pub enum ProgramError {
    #[msg("You are not on the go To jail position.")]
    NotOnGoToJailPosition,
}
