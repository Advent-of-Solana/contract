use anchor_lang::{prelude::*};

#[account]
#[derive(Default,InitSpace)]
pub struct AOS {
    pub authority: Pubkey,
    pub season: u64,
    pub bump: u8,
}
#[account]
#[derive(Default,InitSpace)]
pub struct User {
    pub authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(Default,InitSpace)]
pub struct AOSSubmission {
    pub authority: Pubkey,
    pub solution: [u8;32],
    pub isAccepted: bool,
    pub createdAt: u64,
    pub bump: u8,
}