use anchor_lang::prelude::*;
use anchor_spl::{
    self, associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}
};

use crate::state::Escrow;
#[derive(Accounts)]
#[instruction(seed:u8)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint_a, 
        associated_token::authority = maker
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        space = 8 + Escrow::INIT_SPACE,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive_amount: u64, bump: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(), //public key
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount,
            bump: bump.escrow
        });
        Ok(())
    }

    pub fn deposite(&mut self, desposite: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account = TransferChecked {
            from: self.maker_ata_a.to_account_info(), 
            to: self.maker_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        transfer_checked(cpi_ctx, desposite, self.mint_a.decimals) 
    }
}