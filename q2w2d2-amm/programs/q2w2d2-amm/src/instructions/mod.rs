use anchor_lang::prelude::*;


#[account]
pub struct Config{
    pub seed: u64,
    pub authority: Option<Pubkey>, //someone who can update the config, set rules, lock and unlock the poll
    pub mint_a: Pubkey, //mint of the first token
    pub mint_b: Pubkey, // second token
    pub fee: u16, //liquidity fees
    pub locked: bool, //if the poll is locked, no more votes can be cast
    pub config_bump: u8, //bump seed for the config
    pub lp_bump: u8, //bump seed for the poll

}

impl Space for Config{
    const INIT_SPACE: usize = 8 + 1*2 + 1 +1+2+ 32*2 + 32 + 1 + 8;
    //option keliye 32 + 1 because hoga ya nahi ko store krne me 1 byte.  and pubkey is 32 bytes
}