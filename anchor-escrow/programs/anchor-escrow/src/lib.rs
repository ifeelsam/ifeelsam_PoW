use anchor_lang::prelude::*;

mod instruction;
mod state;

use instruction::*;

declare_id!("FecQQxxdv2NXPGLPB4qxvZvo7QZU1k1RVDXeaud8Ud5");

#[program]
pub mod anchor_escrow {
    use super::*;
    
    pub fn initialize(ctx: Context<Make>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
