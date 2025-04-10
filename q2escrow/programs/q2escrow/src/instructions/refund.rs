use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken,
    token::{TokenAccount, Mint, TransferChecked, transfer_checked},};
    
use crate::state::Escrow;
    
#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}