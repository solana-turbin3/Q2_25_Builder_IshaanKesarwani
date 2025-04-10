#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("8LP2gzGiMPnku4ST49Mm3bx6HjEc95KAgjXyiedtKzKT");

#[program]
pub mod q2vault_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
//this 'info is a lifetime parameter which means that the parameters in the struct exists as long as the struct exists.

    #[account(mut)]
    //this means that the account can be modified
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, seeds = [b"state", user.key().as_ref()],
        bump, 
        space = VaultState::INIT_SPACE,
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Deposit<'info{
    #[account(mut)]
    pub user: Signer <'info>,
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault.vault_bump,
    )]

}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault.vault_bump = bumps.vault;
        Ok(())
    }
}

#[account]
pub struct VaultState{
    pub vault_bump: u8,
    pub state_bump: u8

}

impl Space for VaultState{
    const INIT_SPACE: usize = 8 +1+1;

    //we need to add this 8 bytes always and 1+1 are because of vault_state and vault_bump -> u8 is 8 bits = 1 byte
}