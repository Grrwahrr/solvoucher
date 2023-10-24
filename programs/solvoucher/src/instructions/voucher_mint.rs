use anchor_lang::prelude::*;

use crate::errors;
use crate::state::{Config, OwnerToVoucher, Voucher};
use crate::utils::ConfigState;

#[derive(Accounts)]
#[instruction(config_id: String, data: String)]
pub struct VoucherMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = ["config".as_bytes(), config_id.as_bytes()],
    bump,
    constraint = config.state == ConfigState::Minting
    )]
    pub config: Account<'info, Config>,

    #[account(
    init,
    seeds = ["voucher".as_bytes(), config.key().as_ref(), &config.vouchers_minted.to_le_bytes()],
    bump,
    payer = payer,
    space = Voucher::LEN
    )]
    pub voucher: Account<'info, Voucher>,

    #[account(
    init,
    seeds = ["owner_to_voucher".as_bytes(), config.key().as_ref(), payer.key().as_ref()],
    bump,
    payer = payer,
    space = OwnerToVoucher::LEN
    )]
    pub owner_to_voucher: Account<'info, OwnerToVoucher>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<VoucherMint>, _config_id: String, data: String) -> Result<()> {
    // Will need data from the config
    let config = &mut ctx.accounts.config;

    // Store data on the voucher account
    let voucher = &mut ctx.accounts.voucher;
    voucher.owner = ctx.accounts.payer.key();
    voucher.data = data;

    // Store data on the owner to voucher lookup account
    let owner_to_voucher = &mut ctx.accounts.owner_to_voucher;
    owner_to_voucher.id = config.vouchers_minted;

    // Increase number of vouchers minted
    config.vouchers_minted = config
        .vouchers_minted
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    Ok(())
}
