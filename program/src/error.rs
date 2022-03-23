use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum PollError {
    #[error("Invalid Instruction")]
    InvalidInstruciton,
    #[error("Invalid Instruction Data")]
    InvalidInstrucitonData,
    #[error("Not Initialized")]
    NotInitialized,
    #[error("Pda did not match")]
    PdaNotMatched,
    #[error("Only 255 Polls Supported")]
    PollsOverflow,
    #[error("Poll Already Created")]
    PollAlreadyCreated,
    #[error("Poll Mismatch")]
    PollMismatch,
    #[error("Already Voted")]
    AlreadyVoted,
}

impl From<PollError> for ProgramError {
    fn from(e: PollError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum VoterError {
    #[error("Invalid Instruction")]
    InvalidInstruciton,
    #[error("Already Voted")]
    AlreadyVoted,
    #[error("Vote not initialized")]
    VoteNotInitialized,
    #[error("Vote account does not Match")]
    VoteMismatch,
}

impl From<VoterError> for ProgramError {
    fn from(e: VoterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
