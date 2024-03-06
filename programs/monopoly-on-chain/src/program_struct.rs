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
    pub fn get_next_player(&self) -> Pubkey {
        if self.current_player == 4 {
            return self.players[0];
        }
        self.players[(self.current_player + 1) as usize]
    }
    pub fn get_next_player_index(&self) -> u8 {
        if self.current_player == 4 {
            return 0;
        }
        self.current_player + 1
    }
    pub fn get_current_player(&self) -> Pubkey {
        self.players[self.current_player as usize]
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
impl PlayerState {
    pub fn get_user_position(&self, dice_roll: u8) -> u8 {
        (self.position + dice_roll) % 40
    }
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
    pub rent_levels: [u32; 6],
    pub rent: u32,
}
impl GlobalPositionState {
    pub fn get_rent(&self, house_count: u8, hotel_count: u8) -> u32 {
        if house_count == 0 && hotel_count == 0 {
            return self.rent;
        }
        self.rent_levels[(house_count + hotel_count) as usize]
    }
}

#[derive(Accounts)]
pub struct FinishTurn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, constraint=global_game_state.turn_state == TurnState::UsersMove)]
    pub global_game_state: Account<'info, GlobalGameState>,
    #[account(mut, constraint= global_game_state.get_current_player() == player_state.player && player_state.player == user.key() && player_state.game.key() == global_game_state.key())]
    pub player_state: Account<'info, PlayerState>,
    #[account(mut, constraint=global_position_state.position == player_state.position)]
    pub global_position_state: Account<'info, GlobalPositionState>,
    #[account(mut, constraint= position_game_state.game.key()==global_game_state.key() && global_position_state.position == player_state.position )]
    pub position_game_state: Account<'info, PositionGameState>,
}
impl<'info> FinishTurn<'info> {
    pub fn can_finish_without_paying(&self) -> bool {
        match self.global_position_state.position_type {
            PositionType::Chance
            | PositionType::CommunityChest
            | PositionType::FreeParking
            | PositionType::Go => {
                // These positions do not have ownership or do not require payment to finish the turn.
                return true;
            }
            _ => {}
        }
        if (self.position_game_state.owner == self.player_state.player
            || !self.position_game_state.is_mortgaged)
        {
            return true;
        }
        false
    }
 pub fn get_rent(&self) -> u32 {
        self.global_position_state.get_rent(self.position_game_state.house_count, self.position_game_state.hotel_count)
    }
}
