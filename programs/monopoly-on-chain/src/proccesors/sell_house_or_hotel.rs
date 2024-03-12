use crate::program_struct::*;
use anchor_lang::prelude::*;

pub fn sell_house_or_hotel(ctx: Context<FinishTurn>, sell_hotel: bool) -> Result<()> {
    let position_game_state = &mut ctx.accounts.position_game_state;
    let player_state = &mut ctx.accounts.player_state;
    let global_position_state = &ctx.accounts.global_position_state;

    // Verify the player owns the property and it has the required houses or hotels to sell
    require!(position_game_state.owner == player_state.player, ProgramError::NotPropertyOwner);
    
    if sell_hotel {
        // Verify there's at least one hotel to sell
        require!(position_game_state.hotel_count > 0, ProgramError::NoHotelToSell);
        
        // Decrease the hotel count and increase player's money by half the hotel's cost
        position_game_state.hotel_count -= 1;
        let hotel_sell_price = global_position_state.rent_levels[5] / 2; // Assuming rent_levels[5] holds the hotel purchase price
        player_state.money += hotel_sell_price;
    } else {
        require!(position_game_state.house_count > 0, ProgramError::NoHouseToSell);
        
        position_game_state.house_count -= 1;
        let house_sell_price = global_position_state.price / 2; 
        player_state.money += house_sell_price;
    }

    Ok(())
}

#[error_code]
pub enum ProgramError {
    #[msg("You are not the owner of this property.")]
    NotPropertyOwner,
    #[msg("There is no hotel to sell on this property.")]
    NoHotelToSell,
    #[msg("There is no house to sell on this property.")]
    NoHouseToSell,
}
