use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn mortgage_property(ctx: Context<FinishTurn>) {
    let position_game_state = &mut ctx.accounts.position_game_state;
    let player_state = &mut ctx.accounts.player_state;
    let global_position_state = &ctx.accounts.global_position_state;

    assert!(position_game_state.is_mortgaged == false, "Already Mortgaged");
    assert!(position_game_state.owner == player_state.player, "You are not the owner of this property");

    let mortgage_value = global_position_state.price / 2;

    player_state.money += mortgage_value;

    position_game_state.is_mortgaged = true;
}