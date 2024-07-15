use anchor_lang::prelude::*;

declare_id!("4o2r8vK5pzg6f6wxELeANQqj7Wd3FK1ysnC3EgxzS13t");

#[program] //the whole program and it's functions are defined in this block
pub mod anchor_counter{
    use super::*;

    pub fn initialize(ctx:Context<Initialize>)->Result<()>{
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("counter intialised {}",counter.count);
        Ok(())
    }
    
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>)->Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_sub(1).unwrap();
        msg!("Counter decremented. Current count: {}", counter.count);
        Ok(())
    }
}


#[derive(Accounts)] //all derive(Accounts)  macros are used to define the accounts that are required for the function to run
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

