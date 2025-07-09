use anchor_lang::prelude::*;

declare_id!("YMEAkN6gFcPhRYAHs8qvo1b78X5FPUErFCidpkkmdev");

#[program]
pub mod idl_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.name = "Alice".to_string();
        user.age = 25;
        user.bump = ctx.bumps.user;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 1 + 1 + 100,
        seeds = [b"user"],
        bump
    )]
    pub user: Account<'info, User>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub name: String,
    pub age: u8,
    pub bump: u8,
}