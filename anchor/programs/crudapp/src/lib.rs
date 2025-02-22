#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod crudapp {
    use super::*;
    // create journal entry
    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String ) -> Result<()>{
      let journal_entry: &mut Account<JournalEntryState> = &mut ctx.accounts.journal_entry;

      // set the owner of the journal entry
      journal_entry.owner = *ctx.accounts.owner.key;
      journal_entry.title = title;
      journal_entry.message = message;

      Ok(())
    }

    // create an update journay entry

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()>{
      // logic to fetch update journal entry
      let journal_entry: &mut Account<JournalEntryState> = &mut ctx.accounts.journal_entry;
      journal_entry.message = message; // update the journal entry message
    }
}


#[derive(Accounts)]
#[instruction(title: String message: String)]
pub struct UpdateEntry<'info>{
  #[account(
    mut,
    seed = [title.as_bytes(), owner.key().as_ref()], // this is a neceeary to update the right account
    bump,
    // reallocate the memory for updated string, if it is less, then less rent to pay and vice versa for more characters.
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner, // payer of the realloc is the owner
    realloc::zero = true,   // reset origianl space = zero and do memory reallocation once again
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  // owner of the account paying for the realloc memory
  #[account(mut)]
  pub owner: Signer<'info>,
  pub system_program: Program<'info, System>,  // due to reallocate memory, we need to use system program again to initialize the account
  
}



#[derive(Accounts)] // derive accounts macro
#[instruction(title: String, message: String)] // pass in the title with instruction macro

/** create journal entry struct --> Use for create journal
  * 1. journal_entry: Account to create journal entry state
  * 2. owner: Signer of the account
  * 3. system_program: System program account
  */
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

  // system program account to initialize the account
  pub system_program: Program<'info, System>,

}

#[account] // account macro to create account
#[derive(InitSpace)] // derive init space macro

/** journal entry state struct --> Use for creating journal entry account state.
  * 1. owner: Owner of the journal entry
  * 2. title: Title of the journal entry
  * 3. message: Message of the journal entry
  */
pub struct JournalEntryState{
  pub owner: Pubkey, // owner of the journal entry
  #[max_len(50)] // max length of the title
  pub title: String, // title of the journal entry
  #[max_len(1000)] // max length of the message
  pub message: String, // message of the journal entry
}

