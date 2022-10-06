use thiserror::Error;
// use ethers::prelude::ContractError;

#[derive(Error, Debug, Clone)]
pub enum FlashloanError {
    #[error("Failed to construct call")]
    CallConstructionError(String),
    #[error("Missing token address to borrow")]
    MissingToken,
    #[error("Missing amount to borrow")]
    MissingAmount,
    #[error("Failed to execute contract call")]
    ContractError(dyn Into<dyn std::error::Error>),
}

