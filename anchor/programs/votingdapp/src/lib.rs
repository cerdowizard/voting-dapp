#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
#[allow(dead_code)]
pub mod votingdapp {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, 
        poll_id:u64,
        poll_start:u64,
        poll_end: u64,
        candidates_amount: Option<u64>,
        poll_description: String,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.poll_description = poll_description;
        poll.pool_start = poll_start;
        poll.pool_end = poll_end;
        poll.candidates_amount =0;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
   #[account(mut)]
   pub signers: Signer<'info>,

   #[account(
    init,
    payer = signers,
    space = 8 + Poll::INIT_SPACE,
    seeds = [poll_id.to_le_bytes().as_ref()],
    bump,

   )]
   pub poll:Account<'info, Poll>,

   pub  system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub poll_description: String,
    pub pool_start: u64,
    pub pool_end: u64,
    pub candidates_amount: u64,
}
