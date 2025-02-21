#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod crudapp {
    use super::*;
    // create journal entry
    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, ) -> Result<()>{
      // writing logic to create journal entry 
    }
}



#[derive(Accounts)] // derive accounts macro
#[instruction(title: String)] // pass in the title with instruction macro
// create journal entry struct
pub struct CreateEntry<'info>{
  
  #[account(
    init, // init macro to create account
    seed = [title.as_bytes(), owner.key().as_ref()], // seed to create account, PDA
    bump, // bump to create account after seed
    payer = owner, // Payer of the account is the owner
  )]
  // account to create journal entry state
  pub journal_entry: Account<'info, JournalEntryState>, 

  // owner of the account is the signer
  #[account(mut)]
  pub owner: Signer<'info>,

  // system program account
  pub system_program: Program<'info, System>,

}

#[account] // account macro to create account
#[derive(InitSpace)] // derive init space macro
// journal entry state struct
pub struct JournalEntryState{
  pub owner: Pubkey, // owner of the journal entry
  #[max_len(50)] // max length of the title
  pub title: String, // title of the journal entry
  #[max_len(1000)] // max length of the message
  pub message: String, // message of the journal entry
}

