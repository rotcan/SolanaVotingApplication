use crate::error::PollError;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
//PollCount PDA
//count

//Polling PDA
//poll id u8
//poll text str 100
//poll text len u8
//options (max 4, len 50)
//option_count u8
//bump u8

// Poll Option PDA
// option id u8
// option text str 50
// option text len u8
// option votes u64

//Voter PDA
//pollid u8
//votes u64
//option u8

pub const POLL_TITLE_SIZE: usize = 100;
pub const POLL_OPTION_SIZE: usize = 50;

#[derive(Debug, Clone)]
pub struct PollCount {
    pub is_initialized: bool,
    pub count: u8,
    pub bump: u8,
}

#[derive(Debug, Clone)]
pub struct Poll {
    pub is_initialized: bool,
    pub id: u8,
    pub title: String,
    pub title_length: u8,
    pub options: Vec<PollOption>,
    pub options_count: u8,
    pub bump: u8,
}

#[derive(Debug, Clone)]
pub struct PollOption {
    pub id: u8,
    pub title: String,
    pub title_length: u8,
    pub votes: u64,
}

#[derive(Debug, Clone)]
pub struct PollVoter {
    pub is_initialized: bool,
    pub poll_id: u8,
    pub option_selected: u8,
    pub bump: u8,
}

impl PollCount {
    pub const SIZE: usize = 1 + 1 + 1;
}

impl PollOption {
    pub const SIZE: usize = 1 + POLL_OPTION_SIZE + 1 + 8;

    pub fn new(id: u8, title: String, title_length: u8) -> Self {
        PollOption {
            id,
            title: title,
            title_length,
            votes: 0,
        }
    }
}

impl Poll {
    pub const OPTIONS_COINT: usize = 4;
}

impl Poll {
    pub const SIZE: usize =
        1 + 1 + POLL_TITLE_SIZE + 1 + PollOption::SIZE * Poll::OPTIONS_COINT + 1 + 1;
}

impl PollVoter {
    pub const SIZE: usize = 1 + 1 + 1 + 1;
}

impl Sealed for PollCount {}

impl Sealed for Poll {}

impl Sealed for PollOption {}

impl Sealed for PollVoter {}

impl IsInitialized for Poll {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for PollCount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Poll {
    const LEN: usize = Poll::SIZE;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Poll::LEN];
        let (is_initialized, id, title, title_length, options, options_count, bump) = array_refs![
            src,
            1,
            1,
            POLL_TITLE_SIZE,
            1,
            PollOption::SIZE * Poll::OPTIONS_COINT,
            1,
            1
        ];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let mut options_vec = Vec::new();

        for i in 0..u8::from_le_bytes(*options_count) {
            options_vec.push(Poll::option_at(options, i).unwrap());
        }

        Ok(Poll {
            is_initialized,
            id: u8::from_le_bytes(*id),
            title: String::from_utf8(title.to_vec()).unwrap(),
            title_length: u8::from_le_bytes(*title_length),
            options: options_vec,
            options_count: u8::from_le_bytes(*options_count),
            bump: u8::from_le_bytes(*bump),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Poll::LEN];
        let (
            is_initialized_dst,
            id_dst,
            title_dst,
            title_length_dst,
            options_dst,
            options_count_dst,
            bump_dst,
        ) = mut_array_refs![
            dst,
            1,
            1,
            100,
            1,
            PollOption::SIZE * Poll::OPTIONS_COINT,
            1,
            1
        ];
        let Poll {
            is_initialized,
            id,
            title,
            title_length,
            options,
            options_count,
            bump,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        *id_dst = id.to_le_bytes();
        title_dst.copy_from_slice(title.as_ref());
        *title_length_dst = title_length.to_le_bytes();

        for i in 0..*options_count as usize {
            options.get(i).unwrap().pack_into_slice(
                &mut options_dst[(i * PollOption::SIZE)..(i + 1) * PollOption::SIZE],
            )
        }

        //options_dst.copy_from_slice(options.as_ref());
        *options_count_dst = options_count.to_le_bytes();
        *bump_dst = bump.to_le_bytes();
    }
}

impl<'a> Poll {
    fn option_at(data: &'a [u8], idx: u8) -> Option<PollOption> {
        let op = data
            .get(PollOption::LEN * idx as usize..PollOption::LEN * (idx + 1) as usize)
            .and_then(|slice| slice.try_into().ok())
            .map(|s| PollOption::unpack_unchecked(s))
            .ok_or(None::<PollOption>)
            .unwrap();

        let _ = match op {
            Ok(opt) => return Some(opt),
            _ => return None,
        };
    }

    pub fn add_vote(&mut self, option_id: u8, count: u64) {
        msg!("self.options={:?}", self.options);
        let poll_option = self.options.get_mut(option_id as usize).unwrap();
        poll_option.add_vote(count);
    }
}

impl PollOption {
    pub fn add_vote(&mut self, count: u64) {
        self.votes += count;
    }
}

impl Pack for PollOption {
    const LEN: usize = PollOption::SIZE;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PollOption::LEN];
        let (id, title, title_length, votes) = array_refs![src, 1, POLL_OPTION_SIZE, 1, 8];
        Ok(PollOption {
            id: u8::from_le_bytes(*id),
            title: String::from_utf8(title.to_vec()).unwrap(),
            title_length: u8::from_le_bytes(*title_length),
            votes: u64::from_le_bytes(*votes),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PollOption::LEN];
        let (id_dst, title_dst, title_length_dst, votes_dst) =
            mut_array_refs![dst, 1, POLL_OPTION_SIZE, 1, 8];
        let PollOption {
            id,
            title,
            title_length,
            votes,
        } = self;

        *id_dst = id.to_le_bytes();
        title_dst.copy_from_slice(title.as_ref());
        *title_length_dst = title_length.to_le_bytes();
        *votes_dst = votes.to_le_bytes();
    }
}

impl Pack for PollCount {
    const LEN: usize = PollCount::SIZE;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PollCount::LEN];
        let (is_initialized, count, bump) = array_refs![src, 1, 1, 1];
        //let count = [src[0]];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::from(PollError::NotInitialized)),
        };

        Ok(PollCount {
            is_initialized,
            count: u8::from_le_bytes(*count),
            bump: u8::from_le_bytes(*bump),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        //let dst = &mut [dst[0]];
        let dst = array_mut_ref![dst, 0, PollCount::LEN];
        let (is_initialized_dst, count_dst, bump_dst) = mut_array_refs![dst, 1, 1, 1];
        let PollCount {
            is_initialized,
            count,
            bump,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        *count_dst = count.to_le_bytes();
        *bump_dst = bump.to_le_bytes();
        //*dst = count.to_le_bytes();
    }
}

impl IsInitialized for PollVoter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for PollVoter {
    const LEN: usize = PollVoter::SIZE;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PollVoter::LEN];
        let (is_initialized, poll_id, option_selected, bump) = array_refs![src, 1, 1, 1, 1];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidInstructionData),
        };

        Ok(PollVoter {
            is_initialized,
            poll_id: u8::from_le_bytes(*poll_id),
            option_selected: u8::from_le_bytes(*option_selected),
            bump: u8::from_le_bytes(*bump),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PollVoter::LEN];
        let (is_initialized_dst, poll_id_dst, option_dst, bump_dst) =
            mut_array_refs![dst, 1, 1, 1, 1];
        let PollVoter {
            is_initialized,
            poll_id,
            option_selected,
            bump,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        *poll_id_dst = poll_id.to_le_bytes();
        *option_dst = option_selected.to_le_bytes();
        *bump_dst = bump.to_le_bytes();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VoteCount {
    pub is_initialized: bool,
    // pub owner_key: Pubkey,
    pub vote_group: u8,
    pub vote1: u64,
    pub vote2: u64,
    pub bump: u8,
}

impl VoteCount {
    pub const SIZE: usize = 1 + 1 + 8 + 8 + 1;
}

impl Sealed for VoteCount {}

impl IsInitialized for VoteCount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for VoteCount {
    const LEN: usize = VoteCount::SIZE;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, VoteCount::LEN];
        let (is_initialized, vote_group, vote1, vote2, bump) = array_refs![src, 1, 1, 8, 8, 1];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(VoteCount {
            is_initialized,
            //      owner_key: Pubkey::new_from_array(*owner_key),
            vote_group: u8::from_le_bytes(*vote_group),
            vote1: u64::from_le_bytes(*vote1),
            vote2: u64::from_le_bytes(*vote2),
            bump: u8::from_le_bytes(*bump),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, VoteCount::LEN];
        let (is_initialized_dst, vote_group_dst, vote1_dst, vote2_dst, bump_dst) =
            mut_array_refs![dst, 1, 1, 8, 8, 1];
        let VoteCount {
            is_initialized,
            vote_group,
            vote1,
            vote2,
            bump,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        *vote_group_dst = vote_group.to_le_bytes();
        *vote1_dst = vote1.to_le_bytes();
        *vote2_dst = vote2.to_le_bytes();
        *bump_dst = bump.to_le_bytes();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Voter {
    pub vote_group: u8,
    pub vote_count: u8,
}

impl Sealed for Voter {}
impl Pack for Voter {
    const LEN: usize = 1 + 1;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Voter::LEN];
        let (vote_group, vote_count) = array_refs![src, 1, 1];
        Ok(Voter {
            vote_group: u8::from_le_bytes(*vote_group),
            vote_count: u8::from_le_bytes(*vote_count),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Voter::LEN];
        let (vote_group_dst, vote_count_dst) = mut_array_refs![dst, 1, 1];

        let Voter {
            vote_group,
            vote_count,
        } = self;
        *vote_group_dst = vote_group.to_le_bytes();
        *vote_count_dst = vote_count.to_le_bytes();
    }
}
