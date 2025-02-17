use anchor_lang::prelude::*;

use crate::MarketPlace;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(seeds = [b"marketplace", marketplace.authority.as_ref()], bump = marketplace.bump)]
    pub marketplace: Account<'info, MarketPlace>,
}
