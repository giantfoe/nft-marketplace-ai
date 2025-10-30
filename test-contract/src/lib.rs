use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod test_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Test contract initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}