use thiserror::Error;

/// A Flashloan Builder Error
#[derive(Error, Debug, Clone)]
pub enum FlashloanError {
    /// Failed to create the contract deployer
    #[error("Failed to create flashloan borrower contract deployer!")]
    ContractDeployError,
    /// Failed to deploy the contract
    #[error("Failed to deploy flashloan borrower contract!")]
    ContractDeployFailed,
    /// Missing the flashloan borrower contract
    #[error("Missing flashloan borrower contract definition. Use the `FlashloanBuilder::with_borrower` method to set the contract address or deploy a new instance with `FlashloanBuilder::deploy`")]
    MissingBorrower,
    /// Failed to construct call
    #[error("Failed to construct call: {0}")]
    CallConstructionError(String),
    /// Missing token address
    #[error("Missing token address to borrow. Use the `FlashloanBuilder::with_token` method to set the token address")]
    MissingToken,
    /// Missing amount
    #[error("Missing amount to borrow. Use the `FlashloanBuilder::with_amount` method to set the amount to borrow")]
    MissingAmount,
    /// Failed to execute contract call
    #[error("Failed to execute contract call: {0}")]
    ContractError(String),
    /// Failed to get accounts from web3 provider
    #[error("Failed to get accounts from client. Error: {0}")]
    ClientFailure(String),
    /// Missing the flashloan borrower owner account
    #[error("Missing owner account. Use the `FlashloanBuilder::with_owner` method to set the owner account")]
    MissingOwner,
}

