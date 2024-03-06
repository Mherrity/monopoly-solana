use crate::admin_guard;
use crate::program_struct::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RollDice<'info> {
    #[account(mut,constraint=admin_guard(&admin.to_account_info()))]
    pub admin: Signer<'info>,
    #[account(mut, constraint=global_game_state.turn_state == TurnState::GameStart || global_game_state.turn_state == TurnState::UsersFinished)]
    pub global_game_state: Account<'info, GlobalGameState>,
    #[account(mut, constraint= global_game_state.get_next_player() == player_state.player)]
    pub player_state: Account<'info, PlayerState>,
}

pub fn roll_dice(ctx: Context<RollDice>, roll: u8) {
    let mut global_game_state = &mut ctx.accounts.global_game_state;
    global_game_state.current_player = global_game_state.get_next_player_index();
    global_game_state.turn_state = TurnState::UsersMove;
    let mut player_state = &mut ctx.accounts.player_state;
    //pass go collect 200
    if (player_state.position + roll >= 40) {
        player_state.money += 200;
    }
    player_state.position = player_state.get_user_position(roll);
}
