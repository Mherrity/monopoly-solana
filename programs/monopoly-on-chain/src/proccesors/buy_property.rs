use crate::program_struct::*;
use crate::consts::*;
use anchor_lang::prelude::*;

pub fn buy_property(ctx: Context<FinishTurn>) {
    assert!(BLANK_PUBKEY.parse::<Pubkey>().unwrap() == ctx.accounts.position_game_state.owner, "Property already owned");
    // Check if the player has enough money to buy the property
    let purchase_price = ctx.accounts.global_position_state.price;

    assert!(ctx.accounts.player_state.money < purchase_price, "Not enough money to buy property");

    ctx.accounts.player_state.money -= purchase_price;

    ctx.accounts.position_game_state.owner = ctx.accounts.player_state.player;

}