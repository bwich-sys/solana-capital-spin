use anchor_lang::prelude::*;

#[error_code]
pub enum SpinError {
    #[msg("Count Overflow To Add Item")]
    CountOverflowAddItem,

    #[msg("Index Overflow To Set Item")]
    IndexOverflowSetItem,

    #[msg("Incorrect User State")]
    IncorrectUserState,

    #[msg("Incorrect Claim Amount")]
    ClaimAmountError,
}