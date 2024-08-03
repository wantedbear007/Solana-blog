use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct UserArgs {
  #[max_len(40)]
  pub name: String,
  #[max_len(40)]
  pub email: String,
  #[max_len(10)]
  pub username: String,
  pub authority: Pubkey,
  pub last_blog_id: u8,
  pub total_blog: u8,
}


#[account]
#[derive(Default, InitSpace)]
pub struct BlogContent {
  #[max_len(200)]
  pub description: String,
  #[max_len(100)]
  pub title: String,
  
}