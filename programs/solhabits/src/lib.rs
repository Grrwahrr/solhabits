use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("EXjASzTRvsYBng4TcDKw7xwCPd8KZvAPQgRKqwWUGVpa");

#[program]
pub mod solhabits {
    use super::*;

    #[allow(clippy::result_large_err)]
    pub fn new_habit(
        ctx: Context<NewHabit>,
        amount: u64,
        description: String,
        judge: Pubkey,
        to_success: Pubkey,
        to_failure: Pubkey,
        deadline: u64,
    ) -> Result<()> {
        handle_new_habit(
            ctx,
            amount,
            description,
            judge,
            to_success,
            to_failure,
            deadline,
        )
    }

    #[allow(clippy::result_large_err)]
    pub fn cast_judgement(ctx: Context<CastJudgement>, result: bool) -> Result<()> {
        handle_cast_judgement(ctx, result)
    }

    #[allow(clippy::result_large_err)]
    pub fn clawback(ctx: Context<Clawback>) -> Result<()> {
        handle_clawback(ctx)
    }
}
