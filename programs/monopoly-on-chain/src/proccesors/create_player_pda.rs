use crate::{admin_guard, instruction, program_struct::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreatePlayerPDA<'info> {
    #[account(mut, constraint=global_game_state.is_player(&player.key()))]
    pub player: Signer<'info>,
    #[account(init, payer = player, space = 8 + PlayerState::INIT_SPACE)]
    pub player_pda: Account<'info, PlayerState>,
    #[account(mut,constraint=global_game_state.turn_state == TurnState::GameStart)]
    pub global_game_state: Account<'info, GlobalGameState>,
    pub system_program: Program<'info, System>,
}

pub fn create_player_pda(ctx: Context<CreatePlayerPDA>) {
    let mut player_pda = &mut ctx.accounts.player_pda;
    player_pda.position = 0;
    player_pda.money = 1500;
    player_pda.game = *ctx.accounts.global_game_state.to_account_info().key;
    player_pda.player = *ctx.accounts.player.to_account_info().key;
}
