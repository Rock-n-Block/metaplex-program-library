#![cfg(feature = "test-bpf")]
pub mod common;
pub mod utils;

use common::*;
use solana_sdk::{clock::UnixTimestamp, signature::Keypair, sysvar};
use std::time::SystemTime;
use utils::{helpers::default_scopes, setup_functions::*};

#[tokio::test]
async fn cancel_listing() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(
        &mut context,
        &test_metadata.token.pubkey(),
        100_000_000_000_000,
    )
    .await
    .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();
    context.warp_to_slot(100).unwrap();
    // Derive Auction House Key
    let ((acc, listing_config_address, _), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        10,
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
    let token =
        get_associated_token_address(&test_metadata.token.pubkey(), &test_metadata.mint.pubkey());
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &mpl_gingerbread_house::id());
    let accounts = mpl_gingerbread_house::accounts::GingerbreadHouseCancel {
        auction_house_program: mpl_auction_house::id(),
        auction_house: ahkey,
        wallet: test_metadata.token.pubkey(),
        token_account: token,
        authority: ah.authority,
        trade_state: acc.seller_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority: mpl_gingerbread_house::id(),
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction = Instruction {
        program_id: mpl_gingerbread_house::id(),
        data: mpl_gingerbread_house::instruction::Cancel {
            buyer_price: u64::MAX,
            token_size: 1,
        }
        .data(),
        accounts,
    };

    // let (listing_receipt, _) = find_listing_receipt_address(&acc.seller_trade_state);

    // let accounts = mpl_auction_house::accounts::CancelListingReceipt {
    //     receipt: listing_receipt,
    //     system_program: solana_program::system_program::id(),
    //     instruction: sysvar::instructions::id(),
    // }
    // .to_account_metas(None);
    // let cancel_listing_receipt_instruction = Instruction {
    //     program_id: mpl_auction_house::id(),
    //     data: mpl_auction_house::instruction::CancelListingReceipt {}.data(),
    //     accounts,
    // };

    let tx = Transaction::new_signed_with_payer(
        &[instruction /*, cancel_listing_receipt_instruction*/],
        Some(&test_metadata.token.pubkey()),
        &[&test_metadata.token],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    // let listing_receipt_account = context
    //     .banks_client
    //     .get_account(listing_receipt)
    //     .await
    //     .expect("getting listing receipt")
    //     .expect("empty listing receipt data");

    // let listing_receipt =
    //     ListingReceipt::try_deserialize(&mut listing_receipt_account.data.as_ref()).unwrap();

    // assert_eq!(listing_receipt.canceled_at, Some(timestamp));
    // assert_eq!(listing_receipt.purchase_receipt, None);
}

#[tokio::test]
async fn cancel_bid() {
    let mut context = auctioneer_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    airdrop(&mut context, &test_metadata.token.pubkey(), 1000000000)
        .await
        .unwrap();
    test_metadata
        .create(
            &mut context,
            "Tests".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();

    let price = 1000000000;

    let ((acc, listing_config_address, listing_receipt_acc), sell_tx) = sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        price,
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - 60) as i64,
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

    context.warp_to_slot(100).unwrap();
    let buyer = Keypair::new();
    // Derive Auction House Key
    airdrop(&mut context, &buyer.pubkey(), 2000000000)
        .await
        .unwrap();
    let ((acc, _), buy_tx) = buy(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        &test_metadata.token.pubkey(),
        &buyer,
        &acc.wallet,
        &listing_config_address,
        price,
    );

    context
        .banks_client
        .process_transaction(buy_tx)
        .await
        .unwrap();
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &mpl_gingerbread_house::id());
    let accounts = mpl_gingerbread_house::accounts::GingerbreadHouseCancel {
        auction_house_program: mpl_auction_house::id(),
        auction_house: ahkey,
        wallet: buyer.pubkey(),
        token_account: acc.token_account,
        authority: ah.authority,
        trade_state: acc.buyer_trade_state,
        token_program: spl_token::id(),
        token_mint: test_metadata.mint.pubkey(),
        auction_house_fee_account: ah.auction_house_fee_account,
        auctioneer_authority: mpl_gingerbread_house::id(),
        ah_auctioneer_pda: auctioneer_pda,
    }
    .to_account_metas(None);
    let instruction = Instruction {
        program_id: mpl_gingerbread_house::id(),
        data: mpl_auction_house::instruction::Cancel {
            buyer_price: price,
            token_size: 1,
        }
        .data(),
        accounts,
    };

    // let (bid_receipt, _) = find_bid_receipt_address(&acc.buyer_trade_state);

    // let accounts = mpl_auction_house::accounts::CancelBidReceipt {
    //     receipt: bid_receipt,
    //     system_program: solana_program::system_program::id(),
    //     instruction: sysvar::instructions::id(),
    // }
    // .to_account_metas(None);
    // let cancel_bid_receipt_instruction = Instruction {
    //     program_id: mpl_auction_house::id(),
    //     data: mpl_auction_house::instruction::CancelBidReceipt {}.data(),
    //     accounts,
    // };

    let tx = Transaction::new_signed_with_payer(
        &[instruction /*, cancel_bid_receipt_instruction*/],
        Some(&buyer.pubkey()),
        &[&buyer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    // let bid_receipt_account = context
    //     .banks_client
    //     .get_account(bid_receipt)
    //     .await
    //     .expect("getting bid receipt")
    //     .expect("empty bid receipt data");

    // let bid_receipt = BidReceipt::try_deserialize(&mut bid_receipt_account.data.as_ref()).unwrap();

    // assert_eq!(bid_receipt.canceled_at, Some(timestamp));
    // assert_eq!(bid_receipt.purchase_receipt, None);
}
