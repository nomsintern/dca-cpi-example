use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowErrors {
    #[msg("DCA Account not yet closed")]
    DCANotClosed,

    #[msg("DCA Not Complete")]
    DCANotComplete,

    #[msg("Already airdropped")]
    Airdropped,

    #[msg("Unexpected airdrop amount")]
    UnexpectedAirdropAmount,

    #[msg("Unexpected Balance")]
    UnexpectedBalance,

    #[msg("Insufficient Balance")]
    InsufficientBalance,

    #[msg("Overflow")]
    MathOverflow,

    #[msg("Invalid Plan Parameters")]
    InvalidPlanParameters,
}
