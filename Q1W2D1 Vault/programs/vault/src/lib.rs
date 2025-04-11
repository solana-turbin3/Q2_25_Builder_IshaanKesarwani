use anchor_lang::prelude::*;

declare_id!("AShr8sPKbtRYafAHig3WGHtbGkqRwErbSvywWfzraDZn");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,

//     vault_bump: Used to derive the vault account's PDA
// state_bump: Used to derive the state account's PDA



}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=VaultState::INIT_SPACE+8,
        seeds = [b"state", signer.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds=[vault_state.key().as_ref()],bump)]
    pub vaults: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

    //#[account(seeds=[vault_state.key().as_ref()],bump)]
    // pub vaults: SystemAccount<'info>,

    // References a PDA derived from the vault_state's address
    // This is likely the actual vault that will hold assets
    // SystemAccount indicates this is a system-owned account (not yet initialized)
}

impl<'info> Initialize<'info> {
    pub fn initialize() {
        
    }
}