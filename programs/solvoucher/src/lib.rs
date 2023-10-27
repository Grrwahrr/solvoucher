mod errors;
mod instructions;
mod state;
mod utils;

use crate::utils::ConfigState;
use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("DtDGo9RPBKmVcFKBxBTMzVs6ckQ5bRnrzW1FPYPEEZAq");

#[program]
pub mod solvoucher {
    use super::*;

    /// Setup a new configuration account
    pub fn config_initialize(ctx: Context<ConfigInitialize>, config_id: String) -> Result<()> {
        config_initialize::handler(ctx, config_id)
    }

    /// Update some configuration account
    pub fn config_update(
        ctx: Context<ConfigUpdate>,
        config_id: String,
        state: ConfigState,
    ) -> Result<()> {
        config_update::handler(ctx, config_id, state)
    }

    /// Delete a configuration account
    pub fn config_delete(ctx: Context<ConfigDelete>, config_id: String) -> Result<()> {
        config_delete::handler(ctx, config_id)
    }

    /// Mint a new voucher for a given config account
    pub fn voucher_mint(ctx: Context<VoucherMint>, config_id: String, data: String) -> Result<()> {
        voucher_mint::handler(ctx, config_id, data)
    }

    /// Burn an existing voucher for a given config account
    pub fn voucher_burn(
        ctx: Context<VoucherBurn>,
        config_id: String,
        voucher_id: u32,
    ) -> Result<()> {
        voucher_burn::handler(ctx, config_id, voucher_id)
    }
}
