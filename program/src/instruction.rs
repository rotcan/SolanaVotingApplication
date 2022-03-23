use crate::error::{PollError, VoterError};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::msg;
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum PollInstruction {
    ///
    /// 0, init poll
    ///   create poll count, if it does not exist
    ///   get poll count
    ///   create poll
    ///   create poll options
    ///  accounts
    ///  -poll num account
    ///  -poll account
    ///  -system account
    ///  -payer account
    /// 1, vote poll
    ///  - poll pda account
    ///  - voter pda account
    ///  - voter fee payer account
    ///  - system account
    ///   user votes in poll
    CreatePoll {
        title_length: u8,
        title: String,
        options_count: u8,
        options_size: Vec<u8>,
        options: Vec<String>,
    },
    VotePoll {
        id: u8,
        option_id: u8,
    },
}

impl PollInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::from(PollError::InvalidInstruciton))?;
        match tag {
            0 => PollInstruction::create_poll(rest),
            1 => PollInstruction::vote_poll(rest),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn create_poll(input: &[u8]) -> Result<Self, ProgramError> {
        let mut start_index: usize = 0;
        let title_length = input
            .get(start_index..start_index + 1)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;
        start_index = start_index + 1;
        msg!("title_length={} start_index={}", title_length, start_index);
        let title = input
            .get(start_index..start_index + title_length as usize)
            .and_then(|slice| slice.try_into().ok())
            .map(String::from_utf8)
            .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;
        start_index = start_index + title_length as usize;
        msg!("title={:?} start_index={}", title, start_index);

        let options_count = input
            .get(start_index..start_index + 1)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;
        start_index = start_index + 1;
        msg!(
            "options_count={} start_index={}",
            options_count,
            start_index
        );

        let mut options_str: Vec<String> = Vec::new();
        let mut options_length: Vec<u8> = Vec::new();

        for _ in 0..options_count as usize {
            //
            options_length.push(
                input
                    .get(start_index..start_index + 1)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u8::from_le_bytes)
                    .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?,
            );
            start_index = start_index + 1;
        }

        msg!(
            "options_length={:?} start_index={}",
            options_length,
            start_index
        );
        for i in 0..options_count as usize {
            let opt = input
                .get(start_index..start_index + *options_length.get(i).unwrap() as usize)
                .and_then(|slice| slice.try_into().ok())
                .map(String::from_utf8)
                .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;
            options_str.push(opt.unwrap());
            start_index = start_index + *options_length.get(i).unwrap() as usize;
        }

        msg!("options_str={:?} start_index={}", options_str, start_index);

        Ok(PollInstruction::CreatePoll {
            title_length,
            title: title.unwrap(),
            options_count,
            options_size: options_length,
            options: options_str,
        })
    }

    fn vote_poll(input: &[u8]) -> Result<Self, ProgramError> {
        let poll_id = input
            .get(0..1)
            .and_then(|split| split.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;

        let option_id = input
            .get(1..2)
            .and_then(|split| split.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(ProgramError::from(PollError::InvalidInstrucitonData))?;
        Ok(PollInstruction::VotePoll {
            id: poll_id,
            option_id,
        })
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum VoteInstruction {
    ///
    ///0, signer , account of person creating the counter
    ///1,
    Initialize {
        group: u8,
    },
    Vote {
        group: u8,
        value: u8,
    },
}

impl VoteInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(VoterError::InvalidInstruciton)?;

        Ok(match tag {
            0 => Self::Initialize {
                group: Self::unpack_group(rest)?,
            },
            1 => Self::Vote {
                group: Self::unpack_group(rest)?,
                value: Self::unpack_increment(rest)?,
            },
            _ => return Err(VoterError::InvalidInstruciton.into()),
        })
    }

    fn unpack_group(input: &[u8]) -> Result<u8, ProgramError> {
        let grp = input
            .get(..1)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(VoterError::InvalidInstruciton)?;
        Ok(grp)
    }

    fn unpack_increment(input: &[u8]) -> Result<u8, ProgramError> {
        let inc = input
            .get(1..2)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(VoterError::InvalidInstruciton)?;
        Ok(inc)
    }
}
