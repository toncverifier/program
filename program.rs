use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("HGexHJRXNmKj53ik3BNaKNsE8LgVb7eLe9KAFbASJR8B");

const SEED_SCHEMA: &str = "my seed";

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initiated account: {:?}", ctx.accounts.new_account.to_account_info().key);
        Ok(())
    }

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        msg!("Closing account: {:?}", ctx.accounts.account_to_close.to_account_info().key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        seeds = [
            SEED_SCHEMA.as_bytes(),
            &user.key().as_ref(),              
        ],
        bump,
        space = 8 + 8
    )]
    pub new_account: Box<Account<'info, NewAccount>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut,
        close = user,
        seeds = [
            SEED_SCHEMA.as_bytes(),
            &user.key().as_ref(),              
        ],
        bump
    )]
    pub account_to_close: Box<Account<'info, NewAccount>>,
    pub user: Signer<'info>,
}

#[account]
pub struct NewAccount {
    data: u64
}
