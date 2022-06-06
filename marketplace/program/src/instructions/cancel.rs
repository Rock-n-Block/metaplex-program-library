use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::{prelude::*, AnchorDeserialize, InstructionData};
use anchor_spl::token::{Mint, Token, TokenAccount};

use mpl_auction_house::{
    self, cpi::accounts::AuctioneerCancel as AHCancel,
    program::AuctionHouse as AuctionHouseProgram, AuctionHouse,
};

use crate::constants::{AUCTIONEER, FEE_PAYER, PREFIX};
use crate::utils;

/// Accounts for the [`cancel` handler](auction_house/fn.cancel.html).
#[derive(Accounts)]
#[instruction(auctioneer_authority_bump: u8, buyer_price: u64, token_size: u64)]
pub struct AuctioneerCancel<'info> {
    /// Auction House Program
    pub auction_house_program: Program<'info, AuctionHouseProgram>,

    /// CHECK: Wallet validated as owner in cancel logic.
    /// User wallet account.
    #[account(mut)]
    pub wallet: UncheckedAccount<'info>,

    /// SPL token account containing the token of the sale to be canceled.
    #[account(mut)]
    pub token_account: Box<Account<'info, TokenAccount>>,

    /// Token mint account of SPL token.
    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK: If the AH authority is signer then we sign the auctioneer_authority CPI.
    /// Auction House instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(seeds=[PREFIX.as_ref(), auction_house.creator.as_ref(), auction_house.treasury_mint.as_ref()], seeds::program=auction_house_program, bump=auction_house.bump, has_one=authority, has_one=auction_house_fee_account)]
    pub auction_house: Box<Account<'info, AuctionHouse>>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Auction House instance fee account.
    #[account(mut, seeds=[PREFIX.as_ref(), auction_house.key().as_ref(), FEE_PAYER.as_ref()], seeds::program=auction_house_program, bump=auction_house.fee_payer_bump)]
    pub auction_house_fee_account: UncheckedAccount<'info>,

    /// CHECK: Validated in cancel_logic.
    /// Trade state PDA account representing the bid or ask to be canceled.
    #[account(mut)]
    pub trade_state: UncheckedAccount<'info>,

    /// CHECK: Validated as a signer in cancel_logic.
    /// The auctioneer program PDA running this auction.
    pub auctioneer_authority: UncheckedAccount<'info>,

    /// CHECK: Checked in seed constraints
    /// The auctioneer PDA owned by Auction House storing scopes.
    #[account(seeds = [AUCTIONEER.as_ref(), auction_house.key().as_ref(), auctioneer_authority.key().as_ref()], seeds::program=auction_house_program, bump = auction_house.auctioneer_pda_bump)]
    pub ah_auctioneer_pda: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> From<&mut AuctioneerCancel<'info>> for AHCancel<'info> {
    fn from(accounts: &mut AuctioneerCancel<'info>) -> Self {
        AHCancel {
            wallet: accounts.wallet.to_account_info(),
            token_account: accounts.token_account.to_account_info(),
            token_mint: accounts.token_mint.to_account_info(),
            auction_house: accounts.auction_house.to_account_info(),
            auction_house_fee_account: accounts.auction_house_fee_account.to_account_info(),
            trade_state: accounts.trade_state.to_account_info(),
            authority: accounts.authority.to_account_info(),
            auctioneer_authority: accounts.auctioneer_authority.to_account_info(),
            ah_auctioneer_pda: accounts.ah_auctioneer_pda.to_account_info(),
            token_program: accounts.token_program.to_account_info(),
        }
    }
}

// Cancel a bid or ask by revoking the token delegate, transferring all lamports from the trade state account to the fee payer, and setting the trade state account data to zero so it can be garbage collected.
pub fn auctioneer_cancel(
    ctx: Context<AuctioneerCancel>,
    auctioneer_authority_bump: u8,
    buyer_price: u64,
    token_size: u64,
) -> Result<()> {
    let cpi_program = ctx.accounts.auction_house_program.to_account_info();
    let cpi_accounts: AHCancel = ctx.accounts.into();

    let cancel_data = mpl_auction_house::instruction::AuctioneerCancel {
        buyer_price,
        token_size,
    };

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: cpi_program.key(),
        accounts: utils::to_signed_metas(ctx.accounts.auctioneer_authority.key(), &cpi_accounts),
        data: cancel_data.data(),
    };

    let ah_key = ctx.accounts.auction_house.key();
    let auctioneer_seeds = [
        AUCTIONEER.as_ref(),
        ah_key.as_ref(),
        &[auctioneer_authority_bump],
    ];

    invoke_signed(&ix, &cpi_accounts.to_account_infos(), &[&auctioneer_seeds]).map_err(Into::into)
}
