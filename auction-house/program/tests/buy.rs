//! Module provide tests for `Buy` instruction.

mod utils;
use utils::setup_functions::setup_program;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::Keypair, signer::Signer, system_program,
        sysvar,
    },
    Client, Cluster,
};
use std::error;

#[test]
fn success() -> Result<(), Box<dyn error::Error>> {
    const BUYER_PRICE: u64 = 1;
    const TOKEN_SIZE: u64 = 1;

    // Load `Localnet` keypair
    let wallet = Keypair::new();
    let wallet_pubkey = wallet.pubkey();

    // Initialize anchor RPC `Client`
    let client = Client::new_with_options(
        Cluster::Localnet,
        utils::clone_keypair(&wallet),
        CommitmentConfig::processed(),
    );

    // Initialize `Program` handle
    let program = setup_program(client);

    // Initialize vanilla `RpcClient`
    let connection = program.rpc();

    // Airdrop the payer wallet
    let signature = program
        .rpc()
        .request_airdrop(&program.payer(), 10_000_000_000)?;
    connection.poll_for_signature(&signature)?;

    // Derive native(wrapped) sol mint
    let treasury_mint = spl_token::native_mint::id();

    // Token mint for `TokenMetadata`.
    let token_mint = utils::create_mint(&connection, &wallet)?;

    // Derive / Create associated token account
    let token_account =
        utils::create_associated_token_account(&connection, &wallet, &token_mint.pubkey())?;

    // Mint tokens
    utils::mint_to(
        &connection,
        &wallet,
        &token_mint.pubkey(),
        &token_account,
        1,
    )?;

    // Derive `AuctionHouse` address
    let (auction_house, _) = utils::find_auction_house_address(&wallet_pubkey, &treasury_mint);

    // Derive `AuctionHouse` fee account
    let (auction_house_fee_account, _) =
        utils::find_auction_house_fee_account_address(&auction_house);

    // Derive buyer trade state address
    let (buyer_trade_state, buyer_trade_state_bump) = utils::find_trade_state_address(
        &wallet_pubkey,
        &auction_house,
        &token_account,
        &treasury_mint,
        &token_mint.pubkey(),
        BUYER_PRICE,
        TOKEN_SIZE,
    );

    // Derive escrow payment address
    let (escrow_payment_account, escrow_payment_bump) =
        utils::find_escrow_payment_address(&auction_house, &wallet_pubkey);

    // Create `TokenMetadata`
    let metadata = utils::create_token_metadata(
        &connection,
        &wallet,
        &token_mint.pubkey(),
        String::from("TEST"),
        String::from("TST"),
        String::from("https://github.com"),
        5000,
    )?;

    // Transfer enough lamports to create seller trade state
    utils::transfer_lamports(&connection, &wallet, &auction_house_fee_account, 10000000)?;

    // Perform RPC instruction request
    program
        .request()
        .accounts(mpl_auction_house::accounts::Buy {
            wallet: wallet_pubkey,
            payment_account: wallet_pubkey,
            transfer_authority: wallet_pubkey,
            treasury_mint,
            token_account,
            metadata,
            escrow_payment_account,
            authority: wallet_pubkey,
            auction_house,
            auction_house_fee_account,
            buyer_trade_state,
            token_program: spl_token::id(),
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
        })
        .args(mpl_auction_house::instruction::Buy {
            trade_state_bump: buyer_trade_state_bump,
            escrow_payment_bump,
            buyer_price: BUYER_PRICE,
            token_size: TOKEN_SIZE,
        })
        .send()?;

    assert_eq!(
        connection.get_account_data(&buyer_trade_state)?[0],
        buyer_trade_state_bump
    );

    Ok(())
}