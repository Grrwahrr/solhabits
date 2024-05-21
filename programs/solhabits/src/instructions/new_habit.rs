use anchor_lang::{
    context::Context,
    prelude::{Pubkey, *},
    system_program::System,
    Accounts, Key, Result,
};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::ErrorCode;
use crate::state::habit::Habit;

#[derive(Accounts)]
#[instruction(
amount: u64,
description: String,
judge: Pubkey,
to_success: Pubkey,
to_failure: Pubkey,
deadline: u64,
)]
pub struct NewHabit<'info> {
    /// Who is creating the habit
    #[account(mut)]
    pub signer: Signer<'info>,

    /// Habit PDA
    #[account(
    init,
    seeds = [
    b"habit".as_ref(),
    signer.key().to_bytes().as_ref(),
    description.clone().into_bytes().as_ref()
    ],
    bump,
    space = Habit::LEN,
    payer = signer
    )]
    pub habit: Account<'info, Habit>,

    /// Token account to fund the habit from
    #[account(
    mut,
    token::mint=token_mint,
    token::authority = signer.key()
    )]
    pub token_source: InterfaceAccount<'info, TokenAccount>,

    /// Token account to lock the tokens in
    #[account(
    init,
    associated_token::mint = token_mint,
    associated_token::authority = habit,
    associated_token::token_program = token_program,
    payer = signer,
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    /// The token account mint
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// The associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// The token program
    pub token_program: Interface<'info, TokenInterface>,

    /// The system program
    pub system_program: Program<'info, System>,
}

/// Emitted when a new habit is created
#[event]
pub struct NewHabitEvent {
    /// User creating a habit
    pub creator: Pubkey,

    /// User that get's to do the judging
    pub judge: Pubkey,

    /// The time at which the judge can do their job
    pub deadline: u64,
}

/// Create a new habit
#[allow(clippy::result_large_err)]
pub fn handle_new_habit(
    ctx: Context<NewHabit>,
    amount: u64,
    description: String,
    judge: Pubkey,
    to_success: Pubkey,
    to_failure: Pubkey,
    deadline: u64,
) -> Result<()> {
    // Must transfer some funds
    require!(amount > 0, ErrorCode::AmountIsZero);

    // Deadline must be in the future
    let curr_ts = Clock::get()?.unix_timestamp as u64;
    require!(deadline > curr_ts, ErrorCode::DeadlinePassed);

    // Transfer funds to the token account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.token_source.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        ), //.with_signer(&[&seeds[..]])
        amount,
        ctx.accounts.token_mint.decimals,
    )?;

    // Store data
    let habit = &mut ctx.accounts.habit;
    habit.bump = ctx.bumps.habit;
    habit.creator = ctx.accounts.signer.key();
    habit.description = description;
    habit.judge = judge;
    habit.to_success = to_success;
    habit.to_failure = to_failure;
    habit.deadline = deadline;
    habit.outcome = None;

    // Emit an event
    emit!(NewHabitEvent {
        creator: ctx.accounts.signer.key(),
        judge,
        deadline
    });

    Ok(())
}
