use anchor_lang::prelude::*;
#[doc(hidden)]
pub mod proccesors;
#[doc(hidden)]
pub mod program_struct;
#[doc(hidden)]
pub mod consts;

use std::str::FromStr;

use crate::proccesors::create_global_position::*;
use crate::proccesors::create_player_pda::*;
use crate::proccesors::initialize_game_state::*;
use crate::proccesors::roll_dice::*;
use crate::proccesors::finish_turn::*;
use crate::proccesors::pay_user_and_finish_turn::*;
use crate::proccesors::pay_tax_and_finish_turn::*;
use crate::proccesors::buy_property::*;


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
        rent_levels: [u32; 6],
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
    //rolls the dice once a player finishes their turn
    pub fn roll_dice(ctx: Context<RollDice>, roll: u8) -> Result<()> {
        roll_dice(ctx, roll);
        Ok(())
    }
    //finises the turn of a player (without paying)
    pub fn finish_turn(ctx: Context<FinishTurn>) -> Result<()> {
        finish_turn(ctx);
        Ok(())
    }
    //pays the rent and finishes the turn
    pub fn pay_user_and_finish_turn(ctx: Context<PayUserAndFinishTurn>, payment: u32) -> Result<()> {
        pay_user_and_finish_turn(ctx, payment);
        Ok(())
    }
    //pays the tax and finishes the turn
    pub fn pay_tax_and_finish_turn(ctx: Context<FinishTurn>) -> Result<()> {
        pay_tax_and_finish_turn(ctx);
        Ok(())
    }
    //buys the property
    pub fn buy_property(ctx: Context<FinishTurn>) -> Result<()> {
        buy_property(ctx);
        Ok(())
    }
    //buys the house or hotel
    pub fn buy_house_or_hotel(ctx: Context<FinishTurn>) -> Result<()> {
        buy_house_or_hotel(ctx);
        Ok(())
    }
    //mortgages the house
    pub fn mortgage_house(ctx: Context<FinishTurn>) -> Result<()> {
        mortgage_house(ctx);
        Ok(())
    }
    //lifts the mortgage
    pub fn lift_mortgage(ctx: Context<FinishTurn>) -> Result<()> {
        lift_mortgage(ctx);
        Ok(())
    }
    //sells the house or hotel
    pub fn sell_house_or_hotel(ctx: Context<FinishTurn>) -> Result<()> {
        sell_house_or_hotel(ctx);
        Ok(())
    }
    //sends the player to jail
    pub fn send_player_to_jail(ctx: Context<FinishTurn>) -> Result<()> {
        send_player_to_jail(ctx);
        Ok(())
    }

}

pub const ADMIN_WALLETS: [&str; 1] = ["ADMIN_WALLET"];

pub fn admin_guard(admin: &AccountInfo) -> bool {
    ADMIN_WALLETS
        .iter()
        .any(|w| Pubkey::from_str(w).unwrap() == admin.key())
}
