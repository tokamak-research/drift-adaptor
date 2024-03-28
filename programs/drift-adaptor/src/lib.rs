use anchor_lang::prelude::*;
use instructions::*;

declare_id!("6tTgCX22MLxWBDf4h6yPNEqnshhJDfwetqBdLAfcp2Xi");

#[program]
pub mod drift_adaptor {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        params: InitializeTokenParams,
    ) -> Result<()> {
        instructions::initialize_token(ctx, params)
    }

    pub fn initialize_vault<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeVault<'info>>,
        params: InitializeVaultParams,
    ) -> Result<()> {
        instructions::initialize_vault(ctx, params)
    }
}

mod cpi;
mod error;
mod instructions;
mod macros;
mod states;
