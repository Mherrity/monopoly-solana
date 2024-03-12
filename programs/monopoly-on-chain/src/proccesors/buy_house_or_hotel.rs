use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn buy_house_or_hotel(ctx: Context<FinishTurn>, is_buying_hotel: bool) {
    let property = &mut ctx.accounts.position_game_state;
    let global_property = &ctx.accounts.global_position_state;

    assert!(property.owner == ctx.accounts.player_state.player, "Not your property");
    assert!(property.is_mortgaged == false, "Property is mortgaged");

    let cost = global_property.price;
    assert!(ctx.accounts.player_state.money >= cost, "Your don't have enough money to buy a house/hotel");

    // Deduct the cost from the player's money
    ctx.accounts.player_state.money -= cost;

    // Add a house or convert houses to a hotel
    if is_buying_hotel {
        // Check if there are enough houses to upgrade to a hotel (e.g., 4 houses)
        assert!(property.house_count == 4, "You need 4 houses to upgrade to a hotel");
        property.hotel_count += 1;
        property.house_count = 0; // Reset house count when upgrading to a hotel
    } else {
        // Add a house
        property.house_count += 1;
    }
}