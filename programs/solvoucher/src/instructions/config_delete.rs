use anchor_lang::prelude::*;

use crate::errors;
use crate::state::Config;

#[derive(Accounts)]
#[instruction(config_id: String)]
pub struct ConfigDelete<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
    mut,
    seeds = ["config".as_bytes(), config_id.as_bytes()],
    bump,
    has_one = admin @errors::ErrorCode::NotAuthorized,
    constraint = config.vouchers_minted == config.vouchers_burned @errors::ErrorCode::NotAllVouchersBurned
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(_ctx: Context<ConfigDelete>, _config_id: String) -> Result<()> {
    Ok(())
}
