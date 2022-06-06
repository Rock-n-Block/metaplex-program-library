pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

use anchor_lang::prelude::*;

declare_id!("5GCSoKbYJb47NR8K37hUo4k7T12pXHzJiHy19gBo9yan");

#[program]
pub mod marketplace {

    use super::*;

    /// Authorize the Auctioneer to manage an Auction House.
    pub fn authorize(ctx: Context<AuctioneerAuthorize>) -> Result<()> {
        auctioneer_authorize(ctx)
    }

    /// Withdraw `amount` from the escrow payment account for your specific wallet.
    pub fn withdraw(
        ctx: Context<AuctioneerWithdraw>,
        escrow_payment_bump: u8,
        auctioneer_authority_bump: u8,
        amount: u64,
    ) -> Result<()> {
        auctioneer_withdraw(ctx, escrow_payment_bump, auctioneer_authority_bump, amount)
    }

    /// Deposit `amount` into the escrow payment account for your specific wallet.
    pub fn deposit(
        ctx: Context<AuctioneerDeposit>,
        escrow_payment_bump: u8,
        auctioneer_authority_bump: u8,
        amount: u64,
    ) -> Result<()> {
        auctioneer_deposit(ctx, escrow_payment_bump, auctioneer_authority_bump, amount)
    }

    /// Cancel a bid or ask by revoking the token delegate, transferring all lamports from the trade state account to the fee payer, and setting the trade state account data to zero so it can be garbage collected.
    pub fn cancel(
        ctx: Context<AuctioneerCancel>,
        auctioneer_authority_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        auctioneer_cancel(ctx, auctioneer_authority_bump, buyer_price, token_size)
    }

    /// Create a sell bid by creating a `seller_trade_state` account and approving the program as the token delegate.
    pub fn sell(
        ctx: Context<AuctioneerSell>,
        trade_state_bump: u8,
        free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        auctioneer_authority_bump: u8,
        token_size: u64,
        timed_auction: Option<TimedAuctionArgs>,
        min_bid: u64,
    ) -> Result<()> {
        auctioneer_sell(
            ctx,
            trade_state_bump,
            free_trade_state_bump,
            program_as_signer_bump,
            auctioneer_authority_bump,
            token_size,
            timed_auction,
            min_bid,
        )
    }

    /// Create a private buy bid by creating a `buyer_trade_state` account and an `escrow_payment` account and funding the escrow with the necessary SOL or SPL token amount.
    pub fn buy(
        ctx: Context<AuctioneerBuy>,
        trade_state_bump: u8,
        escrow_payment_bump: u8,
        auctioneer_authority_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        auctioneer_buy(
            ctx,
            trade_state_bump,
            escrow_payment_bump,
            auctioneer_authority_bump,
            buyer_price,
            token_size,
        )
    }

    /// Execute sale between provided buyer and seller trade state accounts transferring funds to seller wallet and token to buyer wallet.
    pub fn execute_sale(
        ctx: Context<AuctioneerExecuteSale>,
        escrow_payment_bump: u8,
        free_trade_state_bump: u8,
        program_as_signer_bump: u8,
        auctioneer_authority_bump: u8,
        buyer_price: u64,
        token_size: u64,
    ) -> Result<()> {
        auctioneer_execute_sale(
            ctx,
            escrow_payment_bump,
            free_trade_state_bump,
            program_as_signer_bump,
            auctioneer_authority_bump,
            buyer_price,
            token_size,
        )
    }
}
