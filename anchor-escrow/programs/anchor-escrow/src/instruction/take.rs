use anchor_lang::prelude::*;
use anchor_spl::{self, associated_token::AssociatedToken, token_2022::transfer_checked, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::state::Escrow;

use super::Make;
#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // #[account(
    //     init_if_needed, 
    //     associated_token::mint = mint_a, 
    //     associated_token::authority = taker
    // )]
    // pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker, 
        associated_token::mint = mint_b, 
        associated_token::authority = taker
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
  pub fn transfer_to_maker(
    &mut self,
  ) -> Result<()> {

      Ok(())
  } 
}