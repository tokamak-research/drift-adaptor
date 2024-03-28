use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use fixed::types::U80F48;

use crate::states::{Price, TokenMap};

pub fn initialize_token(
    ctx: Context<InitializeToken>,
    _params: InitializeTokenParams,
) -> Result<()> {
    let mut price = ctx.accounts.price.load_init()?;
    price.adaptor_token = ctx.accounts.adaptor_token.key();
    price.price = U80F48::from_num(0).into();
    price.last_update = 0;
    drop(price);

    *ctx.accounts.token_map = TokenMap {
        current_size: 8 + 32 + 8 + 24,
        vault: vec![],
    };

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeTokenParams {
    pub decimals: u8,
    pub token_type: u8
}

#[derive(Accounts)]
#[instruction(_params: InitializeTokenParams)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = _params.decimals,
        mint::authority = signer,
        seeds = [base_token.key().as_ref(), &[_params.token_type]],
        bump
    )]
    pub adaptor_token: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + std::mem::size_of::<Price>(),
        seeds = [adaptor_token.key().as_ref()],
        bump
    )]
    pub price: AccountLoader<'info, Price>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 + 24,
        seeds = [b"map", adaptor_token.key().as_ref()],
        bump
    )]
    pub token_map: Account<'info, TokenMap>,

    pub base_token: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
