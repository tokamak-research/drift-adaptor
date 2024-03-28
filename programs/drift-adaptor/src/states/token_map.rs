use anchor_lang::prelude::*;

#[account]
pub struct TokenMap {
    pub current_size: u64,
    pub vault: Vec<Pubkey>,
}