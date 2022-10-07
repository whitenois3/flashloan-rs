use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FlashloanError {
    #[error("Failed to create flashloan borrower contract deployer!")]
    ContractDeployError,
    #[error("Failed to deploy flashloan borrower contract!")]
    ContractDeployFailed,
    #[error("Missing flashloan borrower contract definition. Use the `FlashloanBuilder::with_borrower` method to set the contract address or deploy a new instance with `FlashloanBuilder::deploy`")]
    MissingBorrower,
    #[error("Failed to construct call: {0}")]
    CallConstructionError(String),
    #[error("Missing token address to borrow. Use the `FlashloanBuilder::with_token` method to set the token address")]
    MissingToken,
    #[error("Missing amount to borrow. Use the `FlashloanBuilder::with_amount` method to set the amount to borrow")]
    MissingAmount,
    #[error("Failed to execute contract call: {0}")]
    ContractError(String),
    #[error("Failed to get accounts from client. Error: {0}")]
    ClientFailure(String),
    #[error("Missing owner account. Use the `FlashloanBuilder::with_owner` method to set the owner account")]
    MissingOwner,
}

