use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct Vault {
    pub name: [u8; 32],
    // pub bump: u8,
    // pub _padding: [u8; 31],
    pub base_spot_market_index: u16,
    pub quote_spot_market_index: u16,
    pub base_token: Pubkey,
    pub quote_token: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}

impl Vault {
    pub fn get_vault_signer_seeds<'a>(name: &'a [u8], bump: &'a u8) -> [&'a [u8]; 2] {
        [name, bytemuck::bytes_of(bump)]
    }
}