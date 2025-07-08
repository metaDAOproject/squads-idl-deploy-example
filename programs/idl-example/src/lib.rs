use anchor_lang::prelude::*;

declare_id!("YMEAkN6gFcPhRYAHs8qvo1b78X5FPUErFCidpkkmdev");

#[program]
pub mod idl_example {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct User {
    pub name: String,
    pub age: u8,
    pub bump: u8,
    pub is_admin: bool,
}
