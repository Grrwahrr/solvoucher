use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow Error")]
    OverflowError, // 6000

    #[msg("Not authorized to perform this action")]
    NotAuthorized, // 6001

    #[msg("Can only delete this collection once all vouchers have been burned")]
    NotAllVouchersBurned, // 6002

    #[msg("This collection can not currently be minted")]
    ConfigStateMustBeMinting, // 6003

    #[msg("This collection can not currently be burned")]
    ConfigStateMustBeBurning, // 6004
}
