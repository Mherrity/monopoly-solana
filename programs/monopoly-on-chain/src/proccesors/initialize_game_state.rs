use crate::program_struct::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + GlobalGameState::INIT_SPACE )]
    pub global_game_state: Account<'info, GlobalGameState>,
    pub system_program: Program<'info, System>,
}

pub fn init_global_game_state(ctx: Context<Initialize>, players: [Pubkey; 5]) {
    let mut global_game_state = &mut ctx.accounts.global_game_state;
    global_game_state.players = players;
    global_game_state.current_player = 0;
    global_game_state.turn_state = TurnState::GameStart;
}
