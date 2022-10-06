

// TODO: Test the FlashloanBuilder

#[test]
fn test_create_builder() {
    // Create a flashloan builder
    let mut builder = FlashloanBuilder::new();

    // Make sure it is configured correctly
    assert_eq!(builder.borrower, None);
}