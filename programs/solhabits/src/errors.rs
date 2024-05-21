use anchor_lang::error_code;

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("The deadline must be in the future")]
    DeadlinePassed,

    #[msg("No judgement can be cast before the deadline has passed")]
    DeadlineNotPassed,

    #[msg("Amount can not be zero")]
    AmountIsZero,

    #[msg("Wrong token account")]
    WrongTokenAccount,

    #[msg("Not authorized")]
    NotAuthorized,
}
