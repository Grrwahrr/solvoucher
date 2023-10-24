use anchor_lang::prelude::*;

use crate::state::Config;

#[derive(Accounts)]
#[instruction(config_id: String)]
pub struct ConfigInitialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    init,
    seeds = ["config".as_bytes(), config_id.as_bytes()],
    bump,
    payer = payer,
    space = Config::LEN
    )]
    pub config: Account<'info, Config>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ConfigInitialize>, _config_id: String) -> Result<()> {
    // Store payer as admin
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.payer.key();

    Ok(())
}
