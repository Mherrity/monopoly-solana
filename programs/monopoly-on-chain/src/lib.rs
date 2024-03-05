use anchor_lang::prelude::*;
#[doc(hidden)]
pub mod proccesors;
#[doc(hidden)]
pub mod program_struct;

use std::str::FromStr;

use crate::proccesors::create_global_position::*;
use crate::proccesors::create_player_pda::*;
use crate::proccesors::initialize_game_state::*;
use crate::program_struct::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod monopoly_on_chain {
    use super::*;
    //starts the game for players
    pub fn start_game(ctx: Context<Initialize>, players: [Pubkey; 5]) -> Result<()> {
        init_global_game_state(ctx, players);
        Ok(())
    }

    //these are static positions that hold information about the position on the board
    pub fn create_global_position(
        ctx: Context<CreateGlobalPosition>,
        position: u8,
        position_type: PositionType,
        price: u32,
        rent_levels: [u32; 5],
        rent: u32,
    ) -> Result<()> {
        init_global_position(ctx, position, position_type, price, rent_levels, rent);
        Ok(())
    }

    //creates a player account
    pub fn create_player_pda(ctx: Context<CreatePlayerPDA>) -> Result<()> {
        create_player_pda(ctx);
        Ok(())
    }
}

pub const ADMIN_WALLETS: [&str; 1] = ["ADMIN_WALLET"];

pub fn admin_guard(admin: &AccountInfo) -> bool {
    ADMIN_WALLETS
        .iter()
        .any(|w| Pubkey::from_str(w).unwrap() == admin.key())
}
