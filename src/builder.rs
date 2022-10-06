use ethers::{prelude::{*, builders::ContractCall}, types::transaction, abi::Function};
use std::{str::FromStr, sync::Arc};

/// Flashloan
///
/// ### Usage
///
/// Below we demonstrate a minimal example of how to use the flashloan builder, adding a basic call
/// in the middle of the flashloan.
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
///         
///     );
/// ```
pub struct FlashloanBuilder<M> {
    /// The flashloan borrower contract
    pub borrower: Contract<M>,
    /// The borrower owner (should be the client signer)
    /// If none, the first client account will be used
    pub owner: Option<Address>,
    /// A Middleware Client
    pub client: Arc<M>,
    /// An instance of multicall using [multicall3](https://github.com/mds1/multicall)
    pub multicall: Multicall<M>,
}

impl<M: Middleware> FlashloanBuilder<M> {
    /// Public Associated New Function
    ///
    /// ### Notice
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
    pub async fn new(client: Arc<M>, chain_id: u64) -> Self {
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
                function: Function {},
                block: None,
                client: Arc::clone(&client),
                datatype: std::marker::PhantomData,
            },
        );

        fl
    }

    /// Add a call
    pub fn add_call(&mut self, call: Call) {
        self.multicall.add_call(call);
    }

    /// Execute the flashloan with the wrapped calls
    pub async fn execute(&self) -> Result<Vec<Bytes>, anyhow::Error> {
        self.multicall.send().await
    }
}
