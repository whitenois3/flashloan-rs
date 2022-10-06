use ethers::{prelude::{*, builders::ContractCall}, types::transaction, abi::{Function, Param}};
use std::{str::FromStr, sync::Arc};
use ethers::contract::Contract;

use crate::{calls::Call3, errors::FlashloanError};

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
///     // Create a flashloan builder
///     let mut builder = FlashloanBuilder::new();
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
    pub borrower: Contract<M>,
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
}

impl<M: Middleware> FlashloanBuilder<M> {

    /// Public Associated New Function
    ///
    /// ### Usage
    ///
    /// A [FlashloanBuilder](flashloan_rs::FlashloanBuilder) is constructed using a provided
    /// [Middleware](ethers::Middleware) and u64 chain id.
    ///
    /// Additionally, a [Multicall](ethers::multicall::Multicall) instance is constructed using the
    /// provided [Middleware](ethers::Middleware) and the [Multicall3](https://github.com/mds1/multicall) contract address (`0xcA11bde05977b3631167028862bE2a173976CA11`).
    ///
    /// ### Panics
    ///
    /// Panics if the provided chain id is not valid.
    ///
    /// Technically, panics if Multicall3 contract address is not valid or the Multicall instance
    /// fails to construct. Neither will happen since the contract address is checked and
    /// Multicall3 is backwards-compatible with the Multicall2 and Multicall interfaces.
    pub async fn new(
        client: Arc<M>,
        chain_id: u64,
        override_contract: Option<Address>,
    ) -> Self {
        let mut fl = Self {
            client: Arc::clone(&client),
            multicall: Multicall::new_with_chain_id(
                Arc::clone(&client),
                Some(Address::from_str("0xcA11bde05977b3631167028862bE2a173976CA11").unwrap()),
                Some(U256::from(chain_id).unwrap()),
            )
            .await
            .expect("Failed to initialize Multicall3"),
        };

        // Add the flashloan call
        let sender = client.get_accounts().await.unwrap().get(0).map(|a| *a);

        fl.multicall.add_call(
            ContractCall {
                tx: transaction::eip2718::TypedTransaction::Eip1559(
                    Eip1559TransactionRequest {
                        from: sender,
                        to: (),
                        gas: (),
                        value: (),
                        data: (),
                        nonce: (),
                        access_list: (),
                        max_priority_fee_per_gas: (),
                        max_fee_per_gas: (),
                        chain_id: ()
                    }
                ),
                function: Function {
                    name: "flashBorrow".to_string(),
                    inputs: vec![
                        Param {
                            name: todo!(),
                            kind: todo!(),
                            internal_type: todo!()
                        },
                    ],
                    outputs: todo!(),
                    constant: todo!(),
                    state_mutability: todo!()
                },
                block: None,
                client: Arc::clone(&client),
                datatype: std::marker::PhantomData,
            },
        );

        fl
    }

    /// Deploy a new flashloan borrower contract
    pub fn with_deploy(&self, lender: Option<Address>) -> &self {
        // Get the flash lender
        let deploy_lender = lender.unwrap_or_else(|| {
            self.lender.unwrap_or_else(|| {
                // If none configured, use the MakerDAO Flash Lender
                // This won't panic since the address is checked
                // See: https://github.com/makerdao/dss-flash#deployment
                Address::from_str("0x1eb4cf3a948e7d72a198fe073ccb8c7a948cd853").unwrap()
            })
        });

        Flashloan::deploy(self.client, vec![deploy_lender, ])
        self
    }

    /// Access the inner client
    pub fn inner(&self) -> Arc<M> {
        Arc::clone(&self.client)
    }

    /// Adds a Call3 to the builder
    pub fn add_call(&mut self, call: Call3) {
        self.calls.push(call);
    }

    /// Call
    pub async fn call(&mut self) -> Result<(), FlashloanError> {
        // Deconstruct the flash borrow parameters
        let token = self.token.ok_or(FlashloanError::MissingToken)?;
        let amount = self.amount.ok_or(FlashloanError::MissingAmount)?;
        let mapped_calls = self.calls.iter().map(|c|  builder::flashloan::Call3 {}).collect();
        Flashloan::flash_borrow(&self, token, amount, self.calls).call().await.map_err(|ce| FlashloanError::ContractError(ce))?;
        Ok(())
    }

    /// The internal call executor
    ///
    /// ### Arguments
    ///
    /// Arguments are optional and should be 
    pub fn inner_call(&mut self, token: Option<Address>, amount: Option<U256>, calls: Option<Vec<Call3>>) -> Result<Vec<Bytes>, FlashloanError> {
        
        // Execute the flashloan call
        Flashloan::flash_borrow(&self, token, amount, calls)
        Err(FlashloanError::CallFailed)
    }

    /// Execute the flashloan with the wrapped calls
    pub async fn execute(&self) -> Result<Vec<Bytes>, anyhow::Error> {
        self.multicall.send().await
    }
}
