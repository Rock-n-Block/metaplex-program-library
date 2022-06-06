use anchor_lang::{prelude::*, AnchorDeserialize, InstructionData};
use anchor_spl::token::{Token, TokenAccount};

use mpl_auction_house::{
    self, cpi::accounts::AuctioneerSell as AHSell, program::AuctionHouse as AuctionHouseProgram,
    AuctionHouse,
};

use crate::constants::{AUCTIONEER, FEE_PAYER, LISTING_CONFIG, PREFIX, SIGNER};
use crate::errors::*;
use crate::state::*;
use crate::utils::*;
use anchor_lang::solana_program::program::invoke_signed;

/// Accounts for the [`sell_with_auctioneer` handler](auction_house/fn.sell_with_auctioneer.html).
#[derive(Accounts, Clone)]
#[instruction(trade_state_bump: u8, free_trade_state_bump: u8, program_as_signer_bump: u8, auctioneer_authority_bump: u8, token_size: u64)]
pub struct AuctioneerSell<'info> {
    /// Auction House Program used for CPI call
    pub auction_house_program: Program<'info, AuctionHouseProgram>,

    // Accounts used for Auctioneer
    /// The Listing Config used for listing settings
    #[account(
        init,
        payer=wallet,
        space=ListingConfig::SPACE,
        seeds=[
            LISTING_CONFIG.as_ref(),
            wallet.key().as_ref(),
            auction_house.key().as_ref(),
            token_account.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            token_size.to_le_bytes().as_ref()],
        bump,
    )]
    pub listing_config: Account<'info, ListingConfig>,

    // Accounts passed into Auction House CPI call
    /// CHECK: Verified through CPI
    /// User wallet account.
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// SPL token account containing token for sale.
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Verified through CPI
    /// Metaplex metadata account decorating SPL mint account.
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: Verified through CPI
    /// Auction House authority account.
    pub authority: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds=[
            PREFIX.as_ref(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()],
        seeds::program=auction_house_program,
        bump=auction_house.bump,
        has_one=auction_house_fee_account
    )]
    pub auction_house: Box<Account<'info, AuctionHouse>>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Auction House instance fee account.
    #[account(
        mut,
        seeds=[
            PREFIX.as_ref(),
            auction_house.key().as_ref(),
            FEE_PAYER.as_ref()],
        seeds::program=auction_house_program,
        bump=auction_house.fee_payer_bump
    )]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Seller trade state PDA account encoding the sell order.
    #[account(
        mut,
        seeds=[
            PREFIX.as_ref(),
            wallet.key().as_ref(),
            auction_house.key().as_ref(),
            token_account.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            u64::MAX.to_le_bytes().as_ref(),
            token_size.to_le_bytes().as_ref()],
        seeds::program=auction_house_program,
        bump=trade_state_bump
    )]
    pub seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Free seller trade state PDA account encoding a free sell order.
    #[account(
        mut,
        seeds=[
            PREFIX.as_ref(),
            wallet.key().as_ref(),
            auction_house.key().as_ref(),
            token_account.key().as_ref(),
            auction_house.treasury_mint.as_ref(),
            token_account.mint.as_ref(),
            0u64.to_le_bytes().as_ref(),
            token_size.to_le_bytes().as_ref()],
        seeds::program=auction_house_program,
        bump=free_trade_state_bump
    )]
    pub free_seller_trade_state: UncheckedAccount<'info>,

    /// CHECK: Verified through CPI
    /// The auctioneer program PDA running this auction.
    pub auctioneer_authority: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// The auctioneer PDA owned by Auction House storing scopes.
    #[account(
        seeds = [
            AUCTIONEER.as_ref(),
            auction_house.key().as_ref(),
            auctioneer_authority.key().as_ref()],
        seeds::program=auction_house_program,
        bump = auction_house.auctioneer_pda_bump)]
    pub ah_auctioneer_pda: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(
        seeds=[
            PREFIX.as_ref(),
            SIGNER.as_ref()],
        seeds::program=auction_house_program,
        bump=program_as_signer_bump
    )]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> From<&mut AuctioneerSell<'info>> for AHSell<'info> {
    fn from(accounts: &mut AuctioneerSell<'info>) -> AHSell<'info> {
        AHSell {
            wallet: accounts.wallet.to_account_info(),
            token_account: accounts.token_account.to_account_info(),
            metadata: accounts.metadata.to_account_info(),
            auction_house: accounts.auction_house.to_account_info(),
            auction_house_fee_account: accounts.auction_house_fee_account.to_account_info(),
            seller_trade_state: accounts.seller_trade_state.to_account_info(),
            free_seller_trade_state: accounts.free_seller_trade_state.to_account_info(),
            authority: accounts.authority.to_account_info(),
            auctioneer_authority: accounts.auctioneer_authority.to_account_info(),
            ah_auctioneer_pda: accounts.ah_auctioneer_pda.to_account_info(),
            token_program: accounts.token_program.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
            program_as_signer: accounts.program_as_signer.to_account_info(),
            rent: accounts.rent.to_account_info(),
        }
    }
}

/// Create a sell bid by creating a `seller_trade_state` account and approving the program as the token delegate.
pub fn auctioneer_sell(
    ctx: Context<AuctioneerSell>,
    trade_state_bump: u8,
    free_trade_state_bump: u8,
    program_as_signer_bump: u8,
    auctioneer_authority_bump: u8,
    token_size: u64,
    timed_auction: Option<TimedAuctionArgs>,
    min_bid: u64,
) -> Result<()> {
    require!(min_bid > 0, AuctioneerError::MinBidMusntBeZero);
    // TODO copy decimals from mint account
    ctx.accounts.listing_config.min_bid = min_bid;

    let clock = Clock::get().map_err::<error::Error, _>(Into::into)?;
    let now = clock.unix_timestamp as u64;
    ctx.accounts.listing_config.timed_auction_config = if let Some(args) = timed_auction {
        let start_time = args.start_time.unwrap_or(now);
        require!(
            start_time >= now,
            AuctioneerError::AuctionStartTimeInThePast
        );
        let end_time = args.duration.into();
        Some(TimedAuctionConfig {
            start_time,
            end_time,
        })
    } else {
        None
    };

    ctx.accounts.listing_config.bump = *ctx
        .bumps
        .get("listing_config")
        .ok_or(AuctioneerError::BumpSeedNotInHashMap)?;

    let cpi_program = ctx.accounts.auction_house_program.to_account_info();
    let cpi_accounts: AHSell = ctx.accounts.into();

    let sell_data = mpl_auction_house::instruction::AuctioneerSell {
        trade_state_bump,
        free_trade_state_bump,
        program_as_signer_bump,
        token_size,
    };

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: cpi_program.key(),
        accounts: to_signed_metas(ctx.accounts.auctioneer_authority.key(), &cpi_accounts),
        data: sell_data.data(),
    };

    let ah_key = ctx.accounts.auction_house.key();
    let auctioneer_seeds = [
        AUCTIONEER.as_ref(),
        ah_key.as_ref(),
        &[auctioneer_authority_bump],
    ];

    invoke_signed(&ix, &cpi_accounts.to_account_infos(), &[&auctioneer_seeds]).map_err(Into::into)
}
