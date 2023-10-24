use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Voucher {
    /// The owner of this voucher
    pub owner: Pubkey,

    /// Voucher data
    pub data: String,
}

impl Voucher {
    pub const LEN: usize = std::mem::size_of::<Voucher>() + 8;
}
