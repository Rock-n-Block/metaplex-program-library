use anchor_lang::prelude::*;

use mpl_auction_house::{self, AuctionHouse};

use crate::constants::{AUCTIONEER, PREFIX};

use crate::errors::AuctioneerError;
use crate::state::AuctioneerAuthority;

#[derive(Accounts)]
pub struct AuctioneerAuthorize<'info> {
    /// User wallet account.
    #[account(mut)]
    pub wallet: Signer<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds=[
            PREFIX.as_ref(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()
        ],
        seeds::program=mpl_auction_house::id(),
        bump=auction_house.bump
    )]
    pub auction_house: Box<Account<'info, AuctionHouse>>,

    /// The auctioneer program PDA running this auction.
    #[account(
        init,
        payer=wallet,
        space = 8 + 1,
        seeds = [
            AUCTIONEER.as_ref(),
            auction_house.key().as_ref()
        ], bump)]
    pub auctioneer_authority: Account<'info, AuctioneerAuthority>,

    pub system_program: Program<'info, System>,
}

pub fn auctioneer_authorize(ctx: Context<AuctioneerAuthorize>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.wallet.key(),
        ctx.accounts.auction_house.authority,
        AuctioneerError::SignerNotAuth
    );

    ctx.accounts.auctioneer_authority.bump = *ctx
        .bumps
        .get("auctioneer_authority")
        .ok_or(AuctioneerError::BumpSeedNotInHashMap)?;

    Ok(())
}
