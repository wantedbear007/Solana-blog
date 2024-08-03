use anchor_lang::prelude::*;
use constants::USER_SEED;
use types::UserArgs;

mod types;
mod constants;

declare_id!("9vxgkx6LRkig24EpGehbiuzEf6chPGfUH1CcSpYCzHjq");

#[program]
pub mod blog {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    // to create account
    pub fn create_user(ctx: Context<CreateUser>, name: String, username: String, email: String) -> ProgramResult {
        let user_accounts = &mut ctx.accounts.user_account;

        user_accounts.name = name;
        user_accounts.username = username;
        user_accounts.email = email;
        user_accounts.last_blog_id = 0;
        user_accounts.total_blog = 0;
        user_accounts.authority = ctx.accounts.authority.key();

        Ok(())
    }

    // to create blog
    // pub fn 
}



#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + UserArgs::INIT_SPACE, 
    )]

    pub user_account: Account<'info, UserArgs>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}