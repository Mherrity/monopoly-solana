use crate::program_struct::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PayUserAndFinishTurn<'info> {
    finish_turn_accounts: FinishTurn<'info>,
    #[account(mut, constraint=finish_turn_accounts.position_game_state.owner == owner_player_state.player && finish_turn_accounts.position_game_state.game == owner_player_state.game && !finish_turn_accounts.position_game_state.is_mortgaged )]
    owner_player_state: Account<'info, PlayerState>,
}

pub fn pay_user_and_finish_turn(ctx: Context<PayUserAndFinishTurn>) {
    let rent = ctx.accounts.finish_turn_accounts.get_rent();
    assert!(ctx.accounts.finish_turn_accounts.player_state.money >= rent,"Not enough money to pay rent");
    let mut owner_player_state = &mut ctx.accounts.owner_player_state;
    let mut player_state = &mut ctx.accounts.finish_turn_accounts.player_state;
    owner_player_state.money += rent;
    player_state.money -= rent;
    let mut global_game_state = &mut ctx.accounts.finish_turn_accounts.global_game_state;
    global_game_state.turn_state = TurnState::UsersFinished;
}

