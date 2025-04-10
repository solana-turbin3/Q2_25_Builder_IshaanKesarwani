use anchor_lang::prelude::*;

declare_id!("4tRngai1omNPqiCePqBnoq94H8ELfpkNcPFzRTwnNtZp");

#[program]
pub mod vault {
    use super::*;6
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}



//context -> specifies the accounts which we'll be using
// we will alwasy need an account of type signer, bcz we know atleast one aaccount to pay the fees

#[derive(Accounts)]
pub struct Initialize<'info> {
    // this <'info> sets the lifetime of the account

    #[account(mut)]
        pub signer: Signer<'info>,

    #[account(init, )] //to create account we'll use inti and baaki ka kaam anchor krega
    
}
