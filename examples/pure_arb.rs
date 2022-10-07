use ethers::{
    prelude::*,
    utils::{format_ether, Anvil},
};
use eyre::Result;
use std::{str::FromStr, sync::Arc};

// Import the flashloan-rs crate
use flashloan_rs::prelude::*;

abigen!(DssFlash, "tests/crate/DssFlash.json");

// Run the code asynchronously in a tokio runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Spawn a new anvil instance, forked from mainnet
    let anvil = Anvil::new()
        .fork("https://eth-mainnet.g.alchemy.com/v2/RWYJ7pHUnJFxAROYqC6FfEIDRDmi7luF")
        .spawn();

    // Create a web3 provider
    let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
    let chain_id = provider.get_chainid().await.unwrap().as_u64();

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

    // Deploy the flashloan borrower contract
    println!();
    println!("Deploying flashloan borrower contract...");
    builder.deploy(None, None).await.unwrap();
    println!("Successfully deployed FlashloanBorrower ✅");
    println!();

    // Add calls to the flashloan builder
    // TODO: Swap DAI for ETH via Uniswap
    // TODO: Swap ETH back for DAI via balancer
    // TODO: ensure profit > fee + gas
    builder.add_call(Call3 {
        target: mainnet_dai,
        call_data: Bytes::from(hex::decode("0x095ea7b3").unwrap()),
        value: 0.into(),
        allow_failure: false,
    });

    // TODO: Estimate profitability + gas with a call

    // Then execute
    println!("Executing flashloan...");
    let optional_tx_receipt = builder.execute().await.unwrap();
    assert!(optional_tx_receipt.is_some());
    let tx_receipt = optional_tx_receipt.unwrap();
    println!("Flashloan executed ✅");
    println!();
    println!("TX: https://etherscan.io/tx/{:?}", tx_receipt.transaction_hash);
    println!("[Transaction Hash] {:?}", tx_receipt.transaction_hash);
    println!("[Block Number] {:?}", tx_receipt.block_number.unwrap());
    println!("[Gas Used] {:?}", tx_receipt.gas_used.unwrap());
    println!();

    Ok(())
}
