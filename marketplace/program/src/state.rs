use anchor_lang::prelude::*;

pub const AUCTIONEER_BUYER_PRICE: u64 = u64::MAX;

#[account]
pub struct AuctioneerAuthority {
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TimedAuctionDuration {
    H12 = 12,
    H24 = 24,
    H48 = 48,
}

impl From<TimedAuctionDuration> for u64 {
    fn from(duration: TimedAuctionDuration) -> Self {
        duration as u64 * 60 * 60
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TimedAuctionArgs {
    pub start_time: Option<u64>,
    pub duration: TimedAuctionDuration,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Bid {
    pub amount: u64,
    pub buyer_trade_state: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TimedAuctionConfig {
    pub start_time: u64,
    pub end_time: u64,
}

#[account]
pub struct ListingConfig {
    pub timed_auction_config: Option<TimedAuctionConfig>,
    pub min_bid: u64,
    pub highest_bid: Bid,
    pub bump: u8,
}

impl ListingConfig {
    pub const SPACE: usize = 8 + std::mem::size_of::<ListingConfig>();
}
