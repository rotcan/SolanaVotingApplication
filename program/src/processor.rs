use crate::error::PollError;
use crate::instruction::PollInstruction;
use crate::state::{Poll, PollCount, PollOption, PollVoter, POLL_OPTION_SIZE, POLL_TITLE_SIZE};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program::sysvar::{rent::Rent, Sysvar};
use std::str;

const POLL_COUNT_SEED: &[u8; 9] = b"PollCount";
const POLL_SEED: &[u8; 4] = b"Poll";

pub fn assert_true(cond: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !cond {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        // let instr: VoteInstruction = VoteInstruction::unpack(_instruction_data)?;
        // match instr {
        //     VoteInstruction::Initialize { group } => Self::init_vote(_accounts, group, _program_id),
        //     VoteInstruction::Vote { group, value } => {
        //         Self::vote(_accounts, group, value, _program_id)
        //     }
        // }?;

        let instr: PollInstruction = PollInstruction::unpack(_instruction_data)?;
        msg!("instr={:?}", instr);
        match instr {
            PollInstruction::CreatePoll {
                title_length,
                title,
                options_count,
                options_size,
                options,
            } => Self::create_poll(
                _accounts,
                title_length,
                title,
                options_count,
                options_size,
                options,
                _program_id,
            ),
            PollInstruction::VotePoll { id, option_id } => {
                Self::vote_poll(_accounts, id, option_id, _program_id)
            }
        }?;

        Ok(())
    }

    fn create_poll(
        _acccounts: &[AccountInfo],
        title_length: u8,
        title: String,
        options_count: u8,
        options_size: Vec<u8>,
        options: Vec<String>,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut _acccounts.iter();
        //get poll count pda
        let poll_count_account_iter = next_account_info(accounts_iter)?;
        //get poll pda
        let poll_account_iter = next_account_info(accounts_iter)?;
        //get system program
        let system_program_account = next_account_info(accounts_iter)?;
        //get system program
        let payer_account_iter = next_account_info(accounts_iter)?;
        msg!("before create poll count account");

        //check if poll account is empty
        if poll_count_account_iter.data_is_empty() {
            let (_, bump) = Pubkey::find_program_address(&[POLL_COUNT_SEED], _program_id);
            //create new account
            // payer
            // pda key
            // system program
            invoke_signed(
                &system_instruction::create_account(
                    payer_account_iter.key,
                    poll_count_account_iter.key,
                    Rent::get()?.minimum_balance(PollCount::SIZE),
                    PollCount::SIZE as u64,
                    _program_id,
                ),
                &[
                    payer_account_iter.clone(),
                    poll_count_account_iter.clone(),
                    system_program_account.clone(),
                ],
                &[&[POLL_COUNT_SEED, &[bump]]],
            )?;
        }

        let mut poll_count_account =
            PollCount::unpack_unchecked(&poll_count_account_iter.try_borrow_data()?)?;
        //  {
        //     Ok(acc) => acc,
        //     Err(err) => {
        //         if err == ProgramError::InvalidAccountData {
        //             //get pda
        //             msg!("err={:?}", err);
        //             PollCount::unpack_unchecked(&poll_count_account_iter.try_borrow_data()?)?
        //         } else {
        //             panic!("Something unknown happened!");
        //         }
        //     }
        // };

        msg!("poll_count_account={:?}", poll_count_account);

        if !poll_count_account.is_initialized() {
            //ini poll account
            let (_, bump) = Pubkey::find_program_address(&[POLL_COUNT_SEED], _program_id);

            poll_count_account.is_initialized = true;
            poll_count_account.count = 0;
            poll_count_account.bump = bump;
        }

        let poll_count_pda = Pubkey::create_program_address(
            &[POLL_COUNT_SEED, &[poll_count_account.bump]],
            _program_id,
        )?;

        //check if pda matches the account
        assert_true(
            poll_count_pda == *poll_count_account_iter.key,
            ProgramError::from(PollError::PdaNotMatched),
            "Poll Count account's pda does not match the account passed",
        )?;

        //get next poll number
        poll_count_account.count = poll_count_account.count + 1;
        assert_true(
            poll_count_account.count != 0,
            ProgramError::from(PollError::PollsOverflow),
            "Only 255 polls allowed as of now!",
        )?;

        if poll_account_iter.data_is_empty() {
            let (_, bump) = Pubkey::find_program_address(
                &[POLL_SEED, &[poll_count_account.count]],
                _program_id,
            );

            //create pda by invoke
            invoke_signed(
                &system_instruction::create_account(
                    payer_account_iter.key,
                    poll_account_iter.key,
                    Rent::get()?.minimum_balance(Poll::SIZE),
                    Poll::SIZE as u64,
                    _program_id,
                ),
                &[
                    payer_account_iter.clone(),
                    poll_account_iter.clone(),
                    system_program_account.clone(),
                ],
                &[&[POLL_SEED, &[poll_count_account.count], &[bump]]],
            )?;
        }
        let mut poll_account = Poll::unpack_unchecked(&poll_account_iter.try_borrow_data()?)?;
        //  {
        //     Ok(acc) => acc,
        //     Err(err) => {
        //         if err == ProgramError::InvalidAccountData {
        //             //get pda bump

        //             Poll::unpack_unchecked(&poll_account_iter.try_borrow_data()?)?
        //         } else {
        //             panic!("Something unknown happened!");
        //         }
        //     }
        // };

        assert_true(
            !poll_account.is_initialized(),
            ProgramError::from(PollError::PollAlreadyCreated),
            "Poll already created for this id!",
        )?;

        let (pda, bump) =
            Pubkey::find_program_address(&[POLL_SEED, &[poll_count_account.count]], _program_id);

        if !poll_account.is_initialized() {
            poll_account.is_initialized = true;
            poll_account.id = poll_count_account.count;
            poll_account.title = format!("{:<width$}", title, width = POLL_TITLE_SIZE);
            poll_account.title_length = title_length;
            poll_account.options_count = options_count;
            poll_account.bump = bump;
            poll_account.options = Vec::new();
            //fill empty text
            for i in 0..options_count as usize {
                let sz = *options_size.get(i).unwrap();
                let st = options.get(i).unwrap();
                let po = PollOption::new(
                    i as u8,
                    format!("{:<width$}", &st, width = POLL_OPTION_SIZE),
                    sz,
                );
                poll_account.options.push(po);
            }
            for i in options_count as usize..Poll::OPTIONS_COINT as usize {
                poll_account.options.push(PollOption::new(
                    i as u8,
                    format!("{:<width$}", "", width = POLL_OPTION_SIZE),
                    0,
                ));
            }
            Poll::pack(poll_account, &mut poll_account_iter.try_borrow_mut_data()?)?;
        }

        assert_true(
            *poll_account_iter.key == pda,
            ProgramError::from(PollError::PdaNotMatched),
            "Poll pdas do not match",
        )?;

        PollCount::pack(
            poll_count_account,
            &mut poll_count_account_iter.try_borrow_mut_data()?,
        )?;

        Ok(())
    }

    fn vote_poll(
        _accounts: &[AccountInfo],
        poll_id: u8,
        option_id: u8,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        //accounts
        //poll pda
        //voter pda
        //payer
        //system program
        let accounts_iter = &mut _accounts.iter();
        //poll pda
        let poll_pda_account_iter = next_account_info(accounts_iter)?;
        //voter pda
        let voter_pda_account_iter = next_account_info(accounts_iter)?;
        //voter account
        let voter_iter = next_account_info(accounts_iter)?;
        //system program
        let system_program_account = next_account_info(accounts_iter)?;

        //poll pda
        let mut poll_pda = Poll::unpack_unchecked(&poll_pda_account_iter.try_borrow_data()?)?;

        //get poll account
        let poll_pda_account = Pubkey::create_program_address(
            &[POLL_SEED, &[poll_pda.id], &[poll_pda.bump.clone()]],
            _program_id,
        )?;

        assert_true(
            poll_pda_account == *poll_pda_account_iter.key,
            ProgramError::from(PollError::PdaNotMatched),
            "Pda does not match",
        )?;

        assert_true(
            poll_pda.id == poll_id,
            ProgramError::from(PollError::PollMismatch),
            "Poll account does not match",
        )?;

        assert_true(
            option_id > 0 && option_id <= poll_pda.options_count,
            ProgramError::from(PollError::PollMismatch),
            "Selected option is not present in poll options",
        )?;

        if voter_pda_account_iter.data_is_empty() {
            let (pda, bump) = Pubkey::find_program_address(
                &[
                    POLL_SEED,
                    &[poll_pda.id],
                    &[poll_pda.bump.clone()],
                    voter_iter.key.as_ref(),
                ],
                _program_id,
            );

            msg!("pda={:?}", pda);
            msg!(
                "voter_pda_account_iter.key={:?}",
                voter_pda_account_iter.key
            );

            assert_true(
                pda == *voter_pda_account_iter.key,
                ProgramError::from(PollError::PdaNotMatched),
                "Poll account does not match",
            )?;

            invoke_signed(
                &system_instruction::create_account(
                    voter_iter.key,
                    voter_pda_account_iter.key,
                    Rent::get()?.minimum_balance(PollVoter::SIZE),
                    PollVoter::SIZE as u64,
                    _program_id,
                ),
                &[
                    voter_iter.clone(),
                    voter_pda_account_iter.clone(),
                    system_program_account.clone(),
                ],
                &[&[
                    POLL_SEED,
                    &[poll_pda.id],
                    &[poll_pda.bump.clone()],
                    voter_iter.key.as_ref(),
                    &[bump],
                ]],
            )?;
        }
        //Create Voter Account
        let mut voter_account =
            PollVoter::unpack_unchecked(&voter_pda_account_iter.try_borrow_data()?)?;
        // {
        //     Ok(pda) => pda,
        //     Err(err) => {
        //         if err == ProgramError::InvalidAccountData {
        //             //create voter pda

        //             PollVoter::unpack_unchecked(&voter_pda_account_iter.try_borrow_data()?)?
        //         } else {
        //             panic!("Something went wrong!");
        //         }
        //     }
        // };

        assert_true(
            !voter_account.is_initialized(),
            ProgramError::from(PollError::AlreadyVoted),
            "Already voted for this poll",
        )?;

        let (voter_pda, bump) = Pubkey::find_program_address(
            &[
                POLL_SEED,
                &[poll_pda.id],
                &[poll_pda.bump.clone()],
                voter_iter.key.as_ref(),
            ],
            _program_id,
        );

        voter_account.is_initialized = true;
        voter_account.poll_id = poll_id;
        voter_account.option_selected = option_id;
        voter_account.bump = bump;

        assert_true(
            voter_pda == *voter_pda_account_iter.key,
            ProgramError::from(PollError::PdaNotMatched),
            "Pda does not match",
        )?;

        PollVoter::pack(
            voter_account,
            &mut voter_pda_account_iter.try_borrow_mut_data()?,
        )?;

        poll_pda.add_vote(option_id - 1, 1);
        Poll::pack(poll_pda, &mut poll_pda_account_iter.try_borrow_mut_data()?)?;

        Ok(())
    }

    // fn init_vote(_accounts: &[AccountInfo], group: u8, _program_id: &Pubkey) -> ProgramResult {
    //     let key: &[u8] = b"vote";
    //     let accounts_iter = &mut _accounts.iter();
    //     let (pda_key, bump) = Pubkey::find_program_address(&[key, &[group]], _program_id);

    //     msg!("pda_key={:?} bump={}", pda_key, bump);
    //     msg!("group={}", group);
    //     //probably admin who is going to init the votebank
    //     let payer_account_ai = next_account_info(accounts_iter)?;
    //     //pub key of votebank account
    //     //created using pda
    //     //so program can write to it without fee payer's private key
    //     let votebank_account = next_account_info(accounts_iter)?;
    //     let system_program = next_account_info(accounts_iter)?;
    //     msg!("votebank_account={:?} ", votebank_account.key);
    //     //create votebank account using pda
    //     //so other accounts can update it
    //     invoke_signed(
    //         &system_instruction::create_account(
    //             payer_account_ai.key,
    //             votebank_account.key,
    //             Rent::get()?.minimum_balance(VoteCount::SIZE),
    //             VoteCount::SIZE as u64,
    //             _program_id,
    //         ),
    //         &[
    //             payer_account_ai.clone(),
    //             votebank_account.clone(),
    //             system_program.clone(),
    //         ],
    //         &[&[key, &[group], &[bump]]],
    //         //votebank_account.key.as_ref(),
    //         //payer_account_ai.key.as_ref(),
    //     )?;

    //     let mut votebank = VoteCount::unpack_unchecked(&votebank_account.try_borrow_data()?)?;
    //     assert_true(
    //         !votebank.is_initialized(),
    //         ProgramError::AccountAlreadyInitialized,
    //         "Account is already initialized",
    //     )?;

    //     votebank.is_initialized = true;
    //     votebank.vote1 = 0;
    //     votebank.vote2 = 0;
    //     votebank.vote_group = 1;
    //     votebank.bump = bump;
    //     VoteCount::pack(votebank, &mut votebank_account.try_borrow_mut_data()?)?;

    //     msg!("Initialize");
    //     Ok(())
    // }

    // fn vote(
    //     _accounts: &[AccountInfo],
    //     group: u8,
    //     value: u8,
    //     _program_id: &Pubkey,
    // ) -> ProgramResult {
    //     let key: &[u8] = b"vote";
    //     let accounts_iter = &mut _accounts.iter();
    //     //pub key of voter account
    //     let voter_account = next_account_info(accounts_iter)?;
    //     //vote account
    //     let vote_account = next_account_info(accounts_iter)?;
    //     //pub key of payer
    //     let _ = next_account_info(accounts_iter)?;
    //     msg!("Vote");
    //     //let (pda, bump) = Pubkey::find_program_address(&[key, &[group]], _program_id);
    //     //get voters account
    //     let mut voter = Voter::unpack_unchecked(&voter_account.try_borrow_data()?)?;

    //     //check if vote initialized
    //     let mut vote = VoteCount::unpack_unchecked(&vote_account.try_borrow_data()?)?;
    //     assert_true(
    //         vote.is_initialized(),
    //         ProgramError::from(VoterError::VoteNotInitialized),
    //         "Vote account not initialized",
    //     )?;

    //     let vote_seeds = &[key, &[group], &[vote.bump]]; //vote_account.key.as_ref(),

    //     let vote_account_key = Pubkey::create_program_address(vote_seeds, _program_id)?;
    //     //check if vote account matches the seeds
    //     assert_true(
    //         vote_account_key == *vote_account.key,
    //         ProgramError::from(VoterError::VoteMismatch),
    //         "Vote Account does not match the seeds",
    //     )?;
    //     //check if already voted
    //     assert_true(
    //         voter.vote_group != group,
    //         ProgramError::from(VoterError::AlreadyVoted),
    //         "Already voted for this group",
    //     )?;

    //     //vote for
    //     if value == 1 {
    //         vote.vote1 = vote.vote1 + 1;
    //     } else {
    //         vote.vote2 = vote.vote2 + 1;
    //     }

    //     VoteCount::pack(vote, &mut vote_account.try_borrow_mut_data()?)?;

    //     voter.vote_group = vote.vote_group;
    //     voter.vote_count = value;
    //     Voter::pack(voter, &mut voter_account.try_borrow_mut_data()?)?;

    //     Ok(())
    // }
}
