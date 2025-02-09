use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5yFhFb4wojjy9HYiKv9wS3PuDcizLR22pCN3Jk3vTPGe");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        Ok(())
    }
}
