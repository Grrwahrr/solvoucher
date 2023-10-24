use anchor_lang::prelude::*;

use crate::errors;
use crate::state::{Config, OwnerToVoucher, Voucher};
use crate::utils::ConfigState;

#[derive(Accounts)]
#[instruction(config_id: String, voucher_id: u32)]
pub struct VoucherBurn<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
    mut,
    seeds = ["config".as_bytes(), config_id.as_bytes()],
    bump,
    has_one = admin @errors::ErrorCode::NotAuthorized,
    constraint = config.state == ConfigState::Burning @errors::ErrorCode::ConfigStateMustBeBurning
    )]
    pub config: Account<'info, Config>,

    #[account(
    mut,
    seeds = ["voucher".as_bytes(), config_id.as_bytes(), &voucher_id.to_le_bytes()],
    bump,
    close = admin
    )]
    pub voucher: Account<'info, Voucher>,

    #[account(
    mut,
    seeds = ["owner_to_voucher".as_bytes(), config_id.as_bytes(), voucher.owner.as_ref()],
    bump,
    close = admin
    )]
    pub owner_to_voucher: Account<'info, OwnerToVoucher>,
}

pub fn handler(ctx: Context<VoucherBurn>, _config_id: String, _voucher_id: u32) -> Result<()> {
    // Increase number of vouchers burned
    let config = &mut ctx.accounts.config;
    config.vouchers_burned = config
        .vouchers_burned
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    Ok(())
}
