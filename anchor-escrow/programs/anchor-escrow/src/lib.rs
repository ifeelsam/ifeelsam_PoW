use anchor_lang::prelude::*;

mod state;
mod instructions;
use crate::instructions::*;
// use crate::state::*;

declare_id!("FecQQxxdv2NXPGLPB4qxvZvo7QZU1k1RVDXeaud8Ud5");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u8,
        recieve_amount: u64,
        deposit_amount: u64
    ) -> Result<()> {
        ctx.accounts.init_escrow_account(seed, recieve_amount, &ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.transfer_to_taker()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }
}
