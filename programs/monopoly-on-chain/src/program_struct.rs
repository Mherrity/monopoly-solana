use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace, PartialEq)]

pub enum TurnState {
    UsersMove,
    UsersFinished,
    GameStart,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, InitSpace)]
pub enum PositionType {
    Property,
    Railroad,
    Utility,
    Tax,
    Chance,
    CommunityChest,
    Jail,
    GoToJail,
    FreeParking,
    Go,
}

#[account()]
#[derive(InitSpace)]
pub struct GlobalGameState {
    pub players: [Pubkey; 5],
    pub current_player: u8,
    pub turn_state: TurnState,
}
impl GlobalGameState {
    pub fn is_player(&self, player: &Pubkey) -> bool {
        self.players.iter().any(|p| p == player)
    }
}

#[account]
#[derive(InitSpace)]
pub struct PlayerState {
    pub game: Pubkey,
    pub player: Pubkey,
    pub position: u8,
    pub money: u32,
}

#[account]
#[derive(InitSpace)]
pub struct PositionGameState {
    pub owner: Pubkey,
    pub game: Pubkey,
    pub position: u8,
    pub house_count: u8,
    pub hotel_count: u8,
    pub is_mortgaged: bool,
}

#[account]
#[derive(InitSpace)]
pub struct GlobalPositionState {
    pub position: u8,
    pub position_type: PositionType,
    pub price: u32,
    pub rent_levels: [u32; 5],
    pub rent: u32,
}
