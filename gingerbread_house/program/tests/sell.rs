#![cfg(feature = "test-bpf")]
pub mod common;
pub mod utils;

// use anchor_lang::AccountDeserialize;
use common::*;
use utils::setup_functions::*;

use mpl_auction_house::receipt::ListingReceipt;
use mpl_testing_utils::{solana::airdrop, utils::Metadata};
use solana_sdk::{clock::UnixTimestamp, signer::Signer, sysvar::clock::Clock};
use std::{assert_eq, time::SystemTime};

#[tokio::test]
async fn sell_success() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();
    let ((acc, listing_config_address, listing_receipt_acc), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        1,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()) as i64,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 60) as i64,
    );

    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();
    let sts = context
        .banks_client
        .get_account(acc.seller_trade_state)
        .await
        .expect("Error Getting Trade State")
        .expect("Trade State Empty");
    assert_eq!(sts.data.len(), 1);

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    // let listing_receipt_account = context
    //     .banks_client
    //     .get_account(listing_receipt_acc.receipt)
    //     .await
    //     .expect("getting listing receipt")
    //     .expect("empty listing receipt data");

    // let listing_receipt =
    //     ListingReceipt::try_deserialize(&mut listing_receipt_account.data.as_ref()).unwrap();

    // assert_eq!(listing_receipt.auction_house, acc.auction_house);
    // assert_eq!(listing_receipt.metadata, acc.metadata);
    // assert_eq!(listing_receipt.seller, acc.wallet);
    // assert_eq!(listing_receipt.created_at, timestamp);
    // assert_eq!(listing_receipt.purchase_receipt, None);
    // assert_eq!(listing_receipt.canceled_at, None);
    // assert_eq!(listing_receipt.bookkeeper, *owner_pubkey);
    // assert_eq!(listing_receipt.seller, *owner_pubkey);
    // assert_eq!(listing_receipt.price, 1);
    // assert_eq!(listing_receipt.token_size, 1);

    ()
}