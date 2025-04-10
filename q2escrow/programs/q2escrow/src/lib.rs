use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;


pub use instructions::*;
pub use state::*;
declare_id!("FB4vMuhK7miNpGX9zdXbvp2NHPdk6jkAX4kxdKQGVLmf");

#[program]
pub mod q2escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, receive:u64) -> Result<()> {
        ctx.accounts.deposit();
    }
}

#[derive(Accounts)]
pub struct Initialize {}
