use ethers::prelude::*;
use std::{str::FromStr, sync::Arc};
use anyhow::Result;

use crate::errors::FlashloanError;

#[allow(missing_docs)]
abigen!(Flashloan, "src/FlashBorrower.json");

/// FlashloanBuilder
///
/// ### Usage
///
/// Below we demonstrate a minimal example of how to use the flashloan builder with multicall3 calls.
///
/// ```rust
///     use flashloan_rs::prelude::*;
///     use ethers::prelude::*;
///
///     // Create a default provider
///     let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
///
///     // Create a flashloan builder
///     let mut builder = FlashloanBuilder::new(
///         Arc::clone(&client),
///     );
///
///     // Add a call to the builder
///     builder.add_call(
///         "{
///             \"target\": \"0x0000000000000000000000000000000000000000\",
///             \"allowFailure\": false,
///             \"value\": \"0x0\",
///             \"callData\": \"0x0000000000000000000000000000000000000000000000000000000000000000\"
///         }"
///     );
/// ```
pub struct FlashloanBuilder<M> {
    /// The flashloan borrower contract
    pub borrower: Option<Flashloan<M>>,
    /// The borrower owner (should be the client signer)
    /// If none, the first client account will be used
    pub owner: Option<Address>,
    /// Optional Flash Lender Address
    pub lender: Option<Address>,
    /// A Middleware Client
    pub client: Arc<M>,
    /// The token to borrow
    pub token: Option<Address>,
    /// The amount to borrow
    pub amount: Option<U256>,
    /// Associated calls
    pub calls: Vec<Call3>,
    /// The chain id
    pub chain_id: u64
}

impl<M: Middleware> FlashloanBuilder<M> {

    /// Public Associated New Function
    ///
    /// ### Usage
    ///
    /// A [FlashloanBuilder](flashloan_rs::FlashloanBuilder) is constructed with a given borrower contract if provided.
    ///
    /// ### Arguments
    ///
    /// - `client`: A [Middleware](ethers::Middleware) client
    /// - `chain_id`: A u64 chain id
    /// - `owner`: An optional [Address](ethers::types::Address) for who owns the borrower contract
    /// - `lender`: An optional [Address](ethers::types::Address) for the flash lender
    /// - `token`: An optional [Address](ethers::types::Address) for the token to borrow
    /// - `amount`: An optional [U256](ethers::types::U256) for the amount to borrow
    /// - `override_contract`: Optionally override the flashloan borrower contract to use
    ///
    pub fn new(
        client: Arc<M>,
        chain_id: u64,
        owner: Option<Address>,
        lender: Option<Address>,
        token: Option<Address>,
        amount: Option<U256>,
        override_contract: Option<Address>,
    ) -> Self {
        Self {
            // The borrower contract should be deployed later if not specified
            borrower: override_contract.map(|contract| {
                Flashloan::new(
                    contract,
                    client.clone(),
                )
            }),
            owner,
            lender,
            client: Arc::clone(&client),
            token,
            amount,
            calls: vec![],
            chain_id
        }
    }

    /// Set the owner of the deployed borrower contract
    ///
    /// ### Usage
    ///
    /// This should be set **before** the borrower contract is deployed
    /// by the associated [deploy](FlashBuilder::deploy) method.
    ///
    /// Returns a mutable reference to the builder for method chaining.
    pub fn with_owner(&mut self, owner: Address) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    /// Set the lender of the deployed borrower contract
    ///
    /// ### Usage
    ///
    /// This should be set **before** the borrower contract is deployed
    /// by the associated [deploy](FlashBuilder::deploy) method.
    ///
    /// Returns a mutable reference to the builder for method chaining.
    pub fn with_lender(&mut self, lender: Address) -> &mut Self {
        self.lender = Some(lender);
        self
    }

    /// Deploy a new flashloan borrower contract
    pub async fn deploy(&mut self, lender: Option<Address>, owner: Option<Address>) -> Result<&mut Self> {
        // Unpack the flash lender
        let deploy_lender = lender.unwrap_or_else(|| {
            self.lender.unwrap_or_else(|| {
                // If none configured, use the MakerDAO Flash Lender
                // This won't panic since the address is checked
                // See: https://github.com/makerdao/dss-flash#deployment
                Address::from_str("0x1eb4cf3a948e7d72a198fe073ccb8c7a948cd853").unwrap()
            })
        });

        // Unpack the owner
        let mut optional_owner = owner;
        if optional_owner.is_none() {
            optional_owner = self.owner;
        }
        if optional_owner.is_none() {
            // If none configured, use the first account
            let accounts = self.client.get_accounts().await.map_err(|e| FlashloanError::ClientFailure(e.to_string()))?;
            let first_account = *accounts.get(0).ok_or(FlashloanError::MissingOwner)?;
            optional_owner = Some(first_account);
        }
        let deploy_owner = optional_owner.unwrap();

        let contract_deployer = Flashloan::deploy(Arc::clone(&self.client), vec![deploy_lender, deploy_owner]).map_err(|_| FlashloanError::ContractDeployError)?;
        self.borrower = Some(contract_deployer.send().await.map_err(|_| FlashloanError::ContractDeployError)?);
        Ok(self)
    }

    /// Access the inner client
    pub fn inner(&self) -> Arc<M> {
        Arc::clone(&self.client)
    }

    /// Appends a Call3 to the builder
    /// Returns a reference to the builder for method chaining
    pub fn add_call(&mut self, call: Call3) -> &mut Self {
        self.calls.push(call);
        self
    }

    /// Specify the token address to borrow
    /// Returns a reference to the builder for method chaining
    pub fn with_token(&mut self, token: Address) -> &mut Self {
        self.token = Some(token);
        self
    }

    /// Specify the amount to borrow
    /// Returns a reference to the builder for method chaining
    pub fn with_amount(&mut self, amount: U256) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    /// [**Async**] Call the flashloan function on the borrower contract
    ///
    /// Returns an empty result if successful since the flashloan function should have no return value.
    ///
    /// ### Errors
    ///
    /// Returns a [MissingToken](FlashloanError::MissingToken) if the token address (to borrow) is not specified.
    /// Returns a [MissingAmount](FlashloanError::MissingAmount) if the amount to borrow is not specified.
    /// Returns a [MissingBorrower](FlashloanError::MissingBorrower) if the borrower contract is not specified.
    /// Returns a [ContractError](FlashloanError::ContractError) if the call errors with a string error message.
    pub async fn call(&mut self) -> Result<()> {
        // Deconstruct the flash borrow parameters
        let token = self.token.ok_or(FlashloanError::MissingToken)?;
        let amount = self.amount.ok_or(FlashloanError::MissingAmount)?;
        self.inner_call(token, amount, &*self.calls.clone()).await
    }

    /// The internal call executor
    ///
    /// ### Arguments
    ///
    /// Arguments should be specified with the associated builder pattern methods:
    /// - [with_token](FlashloanBuilder::with_token)
    /// - [with_amount](FlashloanBuilder::with_amount)
    /// - [add_call](FlashloanBuilder::add_call)
    pub async fn inner_call(&mut self, token: Address, amount: U256, calls: &[Call3]) -> Result<()> {
        let contract = self.borrower.as_ref().ok_or(FlashloanError::MissingBorrower)?;
        contract.flash_borrow(token, amount, calls.to_vec()).call().await.map_err(|ce| FlashloanError::ContractError(ce.to_string()))?;
        Ok(())
    }

    /// [**Async**] Execute the flashloan function on the borrower contract
    ///
    /// Returns the result of the flashloan function if successful.
    ///
    /// ### Errors
    ///
    /// Returns a [MissingToken](FlashloanError::MissingToken) if the token address (to borrow) is not specified.
    /// Returns a [MissingAmount](FlashloanError::MissingAmount) if the amount to borrow is not specified.
    /// Returns a [MissingBorrower](FlashloanError::MissingBorrower) if the borrower contract is not specified.
    /// Returns a [ContractError](FlashloanError::ContractError) if the call errors with a string error message.
    pub async fn execute(&mut self) -> Result<Option<TransactionReceipt>> {
        // Deconstruct the flash borrow parameters
        let token = self.token.ok_or(FlashloanError::MissingToken)?;
        let amount = self.amount.ok_or(FlashloanError::MissingAmount)?;
        self.inner_execute(token, amount, &*self.calls.clone()).await
    }

    /// The internal executor
    ///
    /// ### Arguments
    ///
    /// Arguments should be specified with the associated builder pattern methods:
    /// - [with_token](FlashloanBuilder::with_token)
    /// - [with_amount](FlashloanBuilder::with_amount)
    /// - [add_call](FlashloanBuilder::add_call)
    pub async fn inner_execute(&mut self, token: Address, amount: U256, calls: &[Call3]) -> Result<Option<TransactionReceipt>> {
        let contract = self.borrower.as_ref().ok_or(FlashloanError::MissingBorrower)?;
        let contract_call = contract.flash_borrow(token, amount, calls.to_vec());
        let pending_transaction = contract_call.send().await.map_err(|ce| FlashloanError::ContractError(ce.to_string()))?;
        let optional_receipt = pending_transaction.await.map_err(|ce| FlashloanError::ContractError(ce.to_string()))?;
        Ok(optional_receipt)
    }
}
