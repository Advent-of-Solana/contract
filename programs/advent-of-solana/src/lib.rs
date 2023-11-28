use anchor_lang::prelude::*;

declare_id!("uLpQnMfAPKwu5trSGgJgo71Xx65bAgA2nR9HSE6FJLU");

#[program]
pub mod advent_of_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
