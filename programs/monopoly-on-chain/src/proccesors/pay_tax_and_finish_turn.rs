use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn pay_tax_and_finish_turn(ctx: Context<FinishTurn>) {
        if matches!(
            ctx.accounts.global_position_state.position_type,
            PositionType::Tax
        ) {
            let tax_amount = ctx.accounts.global_position_state.rent;
    
            assert!(ctx.accounts.player_state.money < tax_amount, "Not enough money to pay tax"); 
            ctx.accounts.player_state.money -= tax_amount;
    
        }
        ctx.accounts.global_game_state.turn_state = TurnState::UsersFinished;
}

