use anchor_lang::prelude::*;

/// Holds data about a new habit
#[account]
#[derive(Default)]
pub struct Habit {
    /// Bump seed
    pub bump: u8,

    /// Who is creating a new habit
    pub creator: Pubkey,

    /// What's the habit
    pub description: String,

    /// Who will be able to judge the result
    pub judge: Pubkey,

    /// Who receives the tokens on success
    pub to_success: Pubkey,

    /// Who receives the tokens on failure
    pub to_failure: Pubkey,

    /// When is the judge going to be able to make a decision
    pub deadline: u64,

    /// Outcome as judged by the judge
    pub outcome: Option<bool>,
}

impl Habit {
    pub const LEN: usize = 300 + std::mem::size_of::<Habit>();
}
