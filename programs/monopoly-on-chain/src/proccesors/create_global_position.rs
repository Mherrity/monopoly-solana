use crate::{admin_guard, instruction, program_struct::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(position:u8)]
pub struct CreateGlobalPosition<'info> {
    #[account(mut,constraint=admin_guard(&admin.to_account_info()))]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + GlobalPositionState::INIT_SPACE, seeds = [b"position".as_ref(),&[position]], bump)]
    pub global_position_account: Account<'info, GlobalPositionState>,
    pub system_program: Program<'info, System>,
}

pub fn init_global_position(
    ctx: Context<CreateGlobalPosition>,
    position: u8,
    position_type: PositionType,
    price: u32,
    rent_levels: [u32; 6],
    rent: u32,
) {
    let mut global_position_account = &mut ctx.accounts.global_position_account;
    global_position_account.position = position;
    global_position_account.position_type = position_type;
    global_position_account.price = price;
    global_position_account.rent_levels = rent_levels;
    global_position_account.rent = rent;
}
