use anchor_lang::prelude::*;

use crate::errors;
use crate::state::Config;
use crate::utils::ConfigState;

#[derive(Accounts)]
#[instruction(config_id: String, state: ConfigState)]
pub struct ConfigUpdate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
    mut,
    seeds = ["config".as_bytes(), config_id.as_bytes()],
    bump,
    has_one = admin @errors::ErrorCode::NotAuthorized
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfigUpdate>, _config_id: String, state: ConfigState) -> Result<()> {
    // Update the data in the config account
    let config = &mut ctx.accounts.config;
    config.state = state;

    //TODO consider upgradeable admin

    Ok(())
}
