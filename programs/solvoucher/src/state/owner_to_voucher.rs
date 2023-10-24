use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct OwnerToVoucher {
    /// The id of the voucher associated with this account
    pub id: u32,
}

impl OwnerToVoucher {
    pub const LEN: usize = std::mem::size_of::<OwnerToVoucher>() + 8;
}
