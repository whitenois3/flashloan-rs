use ethers::{
    prelude::*,
    utils::{format_ether, Anvil},
};
use std::{str::FromStr, sync::Arc};

use flashloan_rs::prelude::*;

abigen!(DssFlash, "tests/crate/DssFlash.json");

#[test]
fn test_create_builder() {
    // Create a web3 provider
    let client = Provider::<Http>::try_from(
        "https://eth-mainnet.g.alchemy.com/v2/RWYJ7pHUnJFxAROYqC6FfEIDRDmi7luF",
    )
    .unwrap();
    let arc_client = Arc::new(client);

    // Create a flashloan builder
    let builder = FlashloanBuilder::new(Arc::clone(&arc_client), 1, None, None, None, None, None);

    // Make sure it is configured correctly
    assert!(builder.borrower.is_none());
    assert!(builder.owner.is_none());
    assert!(builder.lender.is_none());
    assert!(builder.token.is_none());
    assert!(builder.amount.is_none());
    assert_eq!(builder.calls, vec![]);
    assert_eq!(builder.chain_id, 1);
}

#[tokio::test]
async fn test_deploy_flashloan_borrower() {
    // Spawn a new anvil instance, forked from mainnet
    let anvil = Anvil::new()
        .fork("https://eth-mainnet.g.alchemy.com/v2/RWYJ7pHUnJFxAROYqC6FfEIDRDmi7luF")
        .spawn();
    println!("Spawned anvil instance forked from mainnet");

    // Create a web3 provider
    let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    println!("Anvil instance has chain id {}", chain_id);

    // Grab a local wallet to use as the deployer
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let wallet_address = wallet.address();
    let wallet_balance = provider.get_balance(wallet_address, None).await.unwrap();
    let wallet_balance_ether = format_ether(wallet_balance);
    println!(
        "Using wallet {:?} as deployer [Balance: {} ether]",
        wallet_address, wallet_balance_ether
    );

    let signer = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = Arc::new(signer);

    // Get the makerdao lender contract
    let mainnet_dss_flash =
        Address::from_str("0x1eb4cf3a948e7d72a198fe073ccb8c7a948cd853").unwrap();
    println!("Using DssFlash defined at: {:?}", mainnet_dss_flash);

    // Use DAI as the token to borrow
    let mainnet_dai = Address::from_str("0x6b175474e89094c44da98b954eedeac495271d0f").unwrap();
    println!("Borrowing DAI Token at: {:?}", mainnet_dai);

    // Flashloan the maximum amount
    let dss_flash = DssFlash::new(mainnet_dss_flash, Arc::clone(&client));
    let max_amount = dss_flash.max().call().await.unwrap();

    // Create a flashloan builder
    let mut builder = FlashloanBuilder::new(
        Arc::clone(&client),
        chain_id,
        Some(wallet_address),
        Some(mainnet_dss_flash),
        Some(mainnet_dai),
        Some(max_amount),
        // Don't override contract, we want to deploy
        None,
    );

    // Validate the configuration
    assert!(builder.borrower.is_none());
    assert_eq!(builder.chain_id, chain_id);
    assert_eq!(builder.owner, Some(wallet_address));
    assert_eq!(builder.lender, Some(mainnet_dss_flash));
    assert_eq!(builder.token, Some(mainnet_dai));
    assert_eq!(builder.amount, Some(max_amount));

    // Deploy the flashloan borrower contract
    println!();
    println!("Deploying flashloan borrower contract...");
    builder.deploy(None, None).await.unwrap();
    assert!(builder.borrower.is_some());
    println!("Successfully deployed FlashloanBorrower ✅");
    println!();

    // // Validate the lender and owner are set properly on the deployed flashloan borrower contract
    // let deployed_owner = builder.borrower.unwrap().owner().call().await.unwrap();
    // println!("Deployed flashloan borrower owner: {:?}", deployed_owner);
    // assert_eq!(deployed_owner, wallet_address);
    // let deployed_lender = builder.borrower.unwrap().lender().call().await.unwrap();
    // println!("Deployed flashloan borrower lender: {:?}", deployed_lender);
    // assert_eq!(deployed_lender, mainnet_dss_flash);

    // Call the flashloan function to check if successfull
    println!("Validating flashloan function with static call...");
    builder.call().await.unwrap();
    println!("Successfully called FlashloanBorrower ✅");
    println!();

    // Then execute
    println!("Executing flashloan...");
    let optional_tx_receipt = builder.execute().await.unwrap();
    assert!(optional_tx_receipt.is_some());
    let tx_receipt = optional_tx_receipt.unwrap();
    println!("Flashloan executed ✅");
    println!();
    println!("[Transaction Hash] {:?}", tx_receipt.transaction_hash);
    println!("[Block Number] {:?}", tx_receipt.block_number.unwrap());
    println!("[Gas Used] {:?}", tx_receipt.gas_used.unwrap());
    println!();
}
