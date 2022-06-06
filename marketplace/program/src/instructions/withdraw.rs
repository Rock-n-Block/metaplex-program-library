use anchor_lang::{prelude::*, AnchorDeserialize, InstructionData};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token},
};

use mpl_auction_house::{
    self, cpi::accounts::AuctioneerWithdraw as AHWithdraw,
    program::AuctionHouse as AuctionHouseProgram, AuctionHouse,
};

use crate::constants::{AUCTIONEER, FEE_PAYER, PREFIX};
use crate::utils;
use anchor_lang::solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(escrow_payment_bump: u8, auctioneer_authority_bump: u8)]
pub struct AuctioneerWithdraw<'info> {
    /// Auction House Program
    pub auction_house_program: Program<'info, AuctionHouseProgram>,

    /// CHECK: Verified through CPI
    /// User wallet account.
    pub wallet: UncheckedAccount<'info>,

    /// CHECK: Verified through CPI
    /// SPL token account or native SOL account to transfer funds to. If the account is a native SOL account, this is the same as the wallet address.
    #[account(mut)]
    pub receipt_account: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// Buyer escrow payment account PDA.
    #[account(
        mut,
        seeds=[
            PREFIX.as_ref(),
            auction_house.key().as_ref(),
            wallet.key().as_ref()],
        seeds::program=auction_house_program,
        bump=escrow_payment_bump
    )]
    pub escrow_payment_account: UncheckedAccount<'info>,

    /// Auction House instance treasury mint account.
    pub treasury_mint: Box<Account<'info, Mint>>,

    /// CHECK: Verified through CPI
    /// Auction House instance authority account.
    pub authority: UncheckedAccount<'info>,

    /// Auction House instance PDA account.
    #[account(
        seeds=[
            PREFIX.as_ref(),
            auction_house.creator.as_ref(),
            auction_house.treasury_mint.as_ref()],
        seeds::program=auction_house_program,
        bump=auction_house.bump,
        has_one=treasury_mint,
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

    /// CHECK: Verified through CPI
    /// The auctioneer program PDA running this auction.
    #[account(
        seeds = [
            AUCTIONEER.as_ref(),
            auction_house.key().as_ref()],
        bump=auctioneer_authority_bump
    )]
    pub auctioneer_authority: UncheckedAccount<'info>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    /// The auctioneer PDA owned by Auction House storing scopes.
    #[account(
        seeds = [
            AUCTIONEER.as_ref(),
            auction_house.key().as_ref(),
            auctioneer_authority.key().as_ref()],
        seeds::program=auction_house_program,
        bump = auction_house.auctioneer_pda_bump
    )]
    pub ah_auctioneer_pda: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub ata_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> From<&mut AuctioneerWithdraw<'info>> for AHWithdraw<'info> {
    fn from(accounts: &mut AuctioneerWithdraw<'info>) -> Self {
        AHWithdraw {
            wallet: accounts.wallet.to_account_info(),
            receipt_account: accounts.receipt_account.to_account_info(),
            escrow_payment_account: accounts.escrow_payment_account.to_account_info(),
            treasury_mint: accounts.treasury_mint.to_account_info(),
            authority: accounts.authority.to_account_info(),
            auction_house: accounts.auction_house.to_account_info(),
            auction_house_fee_account: accounts.auction_house_fee_account.to_account_info(),
            auctioneer_authority: accounts.auctioneer_authority.to_account_info(),
            ah_auctioneer_pda: accounts.ah_auctioneer_pda.to_account_info(),
            token_program: accounts.token_program.to_account_info(),
            system_program: accounts.system_program.to_account_info(),
            ata_program: accounts.ata_program.to_account_info(),
            rent: accounts.rent.to_account_info(),
        }
    }
}

pub fn auctioneer_withdraw(
    ctx: Context<AuctioneerWithdraw>,
    escrow_payment_bump: u8,
    auctioneer_authority_bump: u8,
    amount: u64,
) -> Result<()> {
    let cpi_program = ctx.accounts.auction_house.to_account_info();
    let cpi_accounts: AHWithdraw = ctx.accounts.into();

    let withdraw_data = mpl_auction_house::instruction::AuctioneerWithdraw {
        escrow_payment_bump,
        amount,
    };

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: cpi_program.key(),
        accounts: utils::to_signed_metas(ctx.accounts.auctioneer_authority.key(), &cpi_accounts),
        data: withdraw_data.data(),
    };

    let ah_key = ctx.accounts.auction_house.key();
    let auctioneer_seeds = [
        AUCTIONEER.as_ref(),
        ah_key.as_ref(),
        &[auctioneer_authority_bump],
    ];

    invoke_signed(&ix, &cpi_accounts.to_account_infos(), &[&auctioneer_seeds]).map_err(Into::into)
}
