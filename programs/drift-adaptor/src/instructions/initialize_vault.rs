use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use drift::{cpi::accounts::{InitializeUser, InitializeUserStats}, program::Drift};

use crate::{cpi::InitializeUserCPI, error::General, states::{TokenMap, Vault}};

pub fn initialize_vault<'info>(ctx: Context<'_, '_, '_, 'info, InitializeVault<'info>>, params: InitializeVaultParams) -> Result<()> {
    let mut vault = ctx.accounts.vault.load_init()?;
    vault.name = params.name;
    // vault.bump = *ctx.bumps.get("vault").ok_or(General::Default)?;
    vault.base_spot_market_index = params.base_spot_market_index;
    vault.quote_spot_market_index = params.quote_spot_market_index;
    vault.base_token = ctx.accounts.base_token.key();
    vault.quote_token = ctx.accounts.quote_token.key();
    vault.user = ctx.accounts.drift_user.key();
    vault.user_stats = ctx.accounts.drift_user_stats.key();
    drop(vault);

    let mut base_vault_map = ctx.accounts.base_token_map.vault.clone();
    base_vault_map.push(ctx.accounts.vault.key());
    let base_current_size = ctx.accounts.base_token_map.current_size;
    *ctx.accounts.base_token_map = TokenMap {
        current_size: base_current_size + 32,
        vault: base_vault_map,
    };

    let mut quote_vault_map = ctx.accounts.quote_token_map.vault.clone();
    quote_vault_map.push(ctx.accounts.vault.key());
    let quote_current_size = ctx.accounts.quote_token_map.current_size;
    *ctx.accounts.quote_token_map = TokenMap {
        current_size: quote_current_size + 32,
        vault: quote_vault_map,
    };

    let bump = ctx.bumps.get("vault").ok_or(General::Default)?;

    ctx.drift_initialize_user_stats(params.name, *bump)?;
    ctx.drift_initialize_user(params.name, *bump)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: InitializeVaultParams)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [&params.name],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<Vault>(),
    )]
    pub vault: AccountLoader<'info, Vault>,

    pub base_token: Account<'info, Mint>,
    pub quote_token: Account<'info, Mint>,
    #[account(
        seeds = [base_token.key().as_ref(), &[0]],
        bump
    )]
    pub adaptor_base_token: Account<'info, Mint>,
    #[account(
        seeds = [quote_token.key().as_ref(), &[1]],
        bump
    )]
    pub adaptor_quote_token: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"map", adaptor_base_token.key().as_ref()],
        bump,
        realloc = base_token_map.current_size as usize + 32,
        realloc::payer = signer,
        realloc::zero = false
    )]
    pub base_token_map: Account<'info, TokenMap>,

    #[account(
        mut,
        seeds = [b"map", adaptor_quote_token.key().as_ref()],
        bump,
        realloc = quote_token_map.current_size as usize + 32,
        realloc::payer = signer,
        realloc::zero = false
    )]
    pub quote_token_map: Account<'info, TokenMap>,

    // Creation of ATA move to outside of the onchain program, do it in javascript

    // #[account(
    //     init,
    //     payer = signer,
    //     associated_token::mint = base_token,
    //     associated_token::authority = vault
    // )]
    // pub vault_base_token_ata: Account<'info, TokenAccount>,

    // #[account(
    //     init,
    //     payer = signer,
    //     associated_token::mint = quote_token,
    //     associated_token::authority = vault
    // )]
    // pub vault_quote_token_ata: Account<'info, TokenAccount>,

    /// CHECK: checked in drift cpi
    #[account(mut)]
    pub drift_user_stats: AccountInfo<'info>,
    /// CHECK: checked in drift cpi
    #[account(mut)]
    pub drift_user: AccountInfo<'info>,
    /// CHECK: checked in drift cpi
    #[account(mut)]
    pub drift_state: AccountInfo<'info>,
    pub drift_program: Program<'info, Drift>,
    pub rent: Sysvar<'info, Rent>,
    // pub token_program: Program<'info, Token>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeVaultParams {
    pub name: [u8; 32],
    pub base_spot_market_index: u16,
    pub quote_spot_market_index: u16
}


impl<'info> InitializeUserCPI for Context<'_, '_, '_, 'info, InitializeVault<'info>> {
    fn drift_initialize_user_stats(&self, name: [u8; 32], bump: u8) -> Result<()> {
        let signature_seeds = Vault::get_vault_signer_seeds(&name, &bump);
        let signers = &[&signature_seeds[..]];

        let cpi_program = self.accounts.drift_program.to_account_info().clone();
        let cpi_accounts = InitializeUserStats {
            user_stats: self.accounts.drift_user_stats.clone(),
            state: self.accounts.drift_state.clone(),
            authority: self.accounts.vault.to_account_info().clone(),
            payer: self.accounts.signer.to_account_info().clone(),
            rent: self.accounts.rent.to_account_info().clone(),
            system_program: self.accounts.system_program.to_account_info().clone(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers);
        drift::cpi::initialize_user_stats(cpi_ctx)?;

        Ok(())
    }

    fn drift_initialize_user(&self, name: [u8; 32], bump: u8) -> Result<()> {
        let signature_seeds = Vault::get_vault_signer_seeds(&name, &bump);
        let signers = &[&signature_seeds[..]];

        let cpi_program = self.accounts.drift_program.to_account_info().clone();
        let cpi_accounts = InitializeUser {
            user_stats: self.accounts.drift_user_stats.clone(),
            user: self.accounts.drift_user.clone(),
            state: self.accounts.drift_state.clone(),
            authority: self.accounts.vault.to_account_info().clone(),
            payer: self.accounts.signer.to_account_info().clone(),
            rent: self.accounts.rent.to_account_info().clone(),
            system_program: self.accounts.system_program.to_account_info().clone(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers);
        let sub_account_id = 0_u16;
        drift::cpi::initialize_user(cpi_ctx, sub_account_id, name)?;

        Ok(())
    }
}
