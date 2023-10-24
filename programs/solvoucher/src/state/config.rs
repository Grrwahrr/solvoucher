use crate::utils::ConfigState;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Config {
    pub admin: Pubkey,

    /// State of this config
    pub state: ConfigState,

    /// Incremental counter of vouchers created
    pub vouchers_minted: u32,

    /// Incremental counter of vouchers burned
    pub vouchers_burned: u32,
}

impl Config {
    pub const LEN: usize = std::mem::size_of::<Config>() + 8;
}
