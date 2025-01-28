use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint}};
#[derive(Accounts)]
pub struct Refund<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
}