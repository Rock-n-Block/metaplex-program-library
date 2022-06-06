use anchor_lang::prelude::*;

#[error_code]
pub enum AuctioneerError {
    #[msg("Bump seed not in hash map")]
    BumpSeedNotInHashMap,
    #[msg("The signer must be the Auction House authority")]
    SignerNotAuth,
    #[msg("The auction start time can't be in the past")]
    AuctionStartTimeInThePast,
    #[msg("Minimal bid value can't be zero")]
    MinBidMusntBeZero,
    #[msg("Any bid must be a multiple of the bid step - 0.01")]
    IncorrectBidStep,
    #[msg("Auction has not started yet")]
    AuctionNotStarted,
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("The bid was lower than the highest bid")]
    BidTooLow,
    BidStepTooSmall,
    #[msg("Execute Sale must be run on the highest bidder")]
    NotHighestBidder,
    #[msg("Auction has not ended yet")]
    AuctionActive,
}
