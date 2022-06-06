use anchor_lang::prelude::*;

use anchor_spl::token::Mint;

use crate::{errors::*, state::*};

pub fn assert_auction_active(listing_config: &Account<ListingConfig>) -> Result<()> {
    let clock = Clock::get().map_err::<error::Error, _>(Into::into)?;
    let now = clock.unix_timestamp as u64;

    if let Some(TimedAuctionConfig {
        start_time,
        end_time,
    }) = listing_config.timed_auction_config
    {
        require!(now >= start_time, AuctioneerError::AuctionNotStarted);
        require!(now < end_time, AuctioneerError::AuctionEnded);
    }

    Ok(())
}

pub const MIN_BID_STEP_VAL: u64 = 1;
pub const MIN_BID_STEP_DECIMALS: u8 = 2;

pub fn assert_higher_bid(
    listing_config: &Account<ListingConfig>,
    mint: &Box<Account<Mint>>,
    new_bid_price: u64,
) -> Result<()> {
    require!(
        new_bid_price >= listing_config.min_bid,
        AuctioneerError::BidTooLow
    );
    require!(
        new_bid_price >= listing_config.highest_bid.amount,
        AuctioneerError::BidTooLow
    );

    require!(
        mint.decimals >= MIN_BID_STEP_DECIMALS,
        AuctioneerError::BidStepTooSmall
    );
    let min_step = MIN_BID_STEP_VAL * 10_u64.pow((mint.decimals - MIN_BID_STEP_DECIMALS) as u32);
    let diff = listing_config.highest_bid.amount - new_bid_price;
    require!(diff >= min_step, AuctioneerError::BidStepTooSmall);

    Ok(())
}

pub fn assert_auction_over(listing_config: &Account<ListingConfig>) -> Result<()> {
    let clock = Clock::get().map_err::<error::Error, _>(Into::into)?;
    let now = clock.unix_timestamp as u64;

    if let Some(TimedAuctionConfig {
        start_time: _,
        end_time,
    }) = listing_config.timed_auction_config
    {
        require!(now > end_time, AuctioneerError::AuctionActive);
    }

    Ok(())
}

pub fn to_signed_metas<'info, T: ToAccountMetas + ToAccountInfos<'info>>(
    signer: Pubkey,
    cpi_accounts: &T,
) -> Vec<AccountMeta> {
    cpi_accounts
        .to_account_metas(None)
        .into_iter()
        .zip(cpi_accounts.to_account_infos())
        .map(|mut pair| {
            pair.0.is_signer = pair.1.is_signer;
            if pair.0.pubkey == signer {
                pair.0.is_signer = true;
            }
            pair.0
        })
        .collect()
}
