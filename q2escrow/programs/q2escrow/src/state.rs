use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub seed: u64,
    pub marker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64, 
    //receive is the recv amount
    pub bumo: u8,

}