use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn finish_turn(ctx: Context<FinishTurn>) {
    assert!(ctx.accounts.can_finish_without_paying(), "You must pay to finish your turn");
    let mut global_game_state = &mut ctx.accounts.global_game_state;
    global_game_state.turn_state = TurnState::UsersFinished;
}
