use anchor_lang::{
    context::Context,
    prelude::{Pubkey, *},
    system_program::System,
    Accounts, Key, Result,
};
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::ErrorCode;
use crate::state::habit::Habit;

#[derive(Accounts)]
pub struct CastJudgement<'info> {
    /// Who is judging the outcome
    #[account(mut)]
    pub judge: Signer<'info>,

    /// Habit PDA
    #[account(
    mut,
    constraint = habit.judge == judge.key(),
    constraint = habit.outcome == None,
    )]
    pub habit: Account<'info, Habit>,

    /// Token account with the locked funds
    #[account(
    mut,
    token::mint=token_mint,
    token::authority = habit.key()
    )]
    pub token_vault: InterfaceAccount<'info, TokenAccount>,

    /// Token account to transfer the locked funds to
    #[account(mut)]
    pub token_destination: InterfaceAccount<'info, TokenAccount>,

    /// The token account mint
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// The token program
    pub token_program: Interface<'info, TokenInterface>,

    /// The system program
    pub system_program: Program<'info, System>,
}

/// Emitted when a habit is judged
#[event]
pub struct HabitJudgedEvent {
    /// The habit
    pub habit: Pubkey,

    /// User who created it
    pub creator: Pubkey,

    /// User that gets to do the judging
    pub judge: Pubkey,

    /// The outcome according to the judge
    pub result: bool,
}

/// The judge decides whether the habit is a success or not
#[allow(clippy::result_large_err)]
pub fn handle_cast_judgement(ctx: Context<CastJudgement>, result: bool) -> Result<()> {
    // Need to access the habit state
    let habit = &mut ctx.accounts.habit;

    // Update the outcome
    habit.outcome = Some(result);

    // Deadline must have passed
    let curr_ts = Clock::get()?.unix_timestamp as u64;
    require!(habit.deadline <= curr_ts, ErrorCode::DeadlineNotPassed);

    // Make sure the destination address is correct
    let destination = if result {
        habit.to_success
    } else {
        habit.to_failure
    };
    let destination_ata =
        get_associated_token_address(&destination, &ctx.accounts.token_mint.key());
    require!(
        destination_ata == ctx.accounts.token_destination.key(),
        ErrorCode::WrongTokenAccount
    );

    //TODO should the ATA be made here or bundled by FE ?
    // if ctx.accounts.token_destination.lamports() == 0 {
    //     create_associated_token_account(
    //         CpiContext::new(
    //             ctx.accounts.associated_token_program.to_account_info(),
    //             Create {
    //                 payer: payer.clone(),
    //                 associated_token: destination.clone(),
    //                 authority: destination,
    //                 mint: ctx.accounts.token_mint.to_account_info(),
    //                 token_program: token_program.clone(),
    //                 rent: ctx.accounts.rent.to_account_info(),
    //                 system_program: ctx.accounts.system_program.to_account_info(),
    //             },
    //         ),
    //     )?;
    // }

    // Create signer seeds from habit PDA
    let seeds = [
        b"habit".as_ref(),
        &habit.creator.to_bytes(),
        &habit.description.clone().into_bytes(),
        &[ctx.accounts.habit.bump],
    ];

    // Transfer funds to the token account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.token_destination.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                authority: ctx.accounts.habit.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        ctx.accounts.token_vault.amount,
        ctx.accounts.token_mint.decimals,
    )?;

    // Close token vault account if possible
    //TODO

    // Emit an event
    emit!(HabitJudgedEvent {
        habit: ctx.accounts.habit.key(),
        creator: ctx.accounts.habit.creator,
        judge: ctx.accounts.habit.judge,
        result
    });

    Ok(())
}
