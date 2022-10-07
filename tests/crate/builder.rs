use std::sync::Arc;
use ethers::prelude::*;
use flashloan_rs::prelude::*;

#[test]
fn test_create_builder() {
    // Create a web3 provider
    let client = Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").unwrap();
    let arc_client = Arc::new(client);

    // Create a flashloan builder
    let builder = FlashloanBuilder::new(
        Arc::clone(&arc_client),
        1,
        None,
        None,
        None,
        None,
        None
    );

    // Make sure it is configured correctly
    assert!(builder.borrower.is_none());
    assert!(builder.owner.is_none());
    assert!(builder.lender.is_none());
    assert!(builder.token.is_none());
    assert!(builder.amount.is_none());
    assert_eq!(builder.calls, vec![]);
    assert_eq!(builder.chain_id, 1);
}

#[test]
fn test_create_full_builder() {
    // Create a web3 provider
    let client = Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27").unwrap();
    let arc_client = Arc::new(client);

    // Create a flashloan borrower owner
    let owner = Address::from_str("0x0000000000000000000000000000000000000000").unwrap();

    // Create a flashloan builder
    let builder = FlashloanBuilder::new(
        Arc::clone(&arc_client),
        1,
        None,
        None,
        None,
        None,
        None
    );

    // Make sure it is configured correctly
    assert!(builder.borrower.is_none());
    assert!(builder.owner.is_none());
    assert!(builder.lender.is_none());
    assert!(builder.token.is_none());
    assert!(builder.amount.is_none());
    assert_eq!(builder.calls, vec![]);
    assert_eq!(builder.chain_id, 1);
}