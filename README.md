<img align="right" width="150" height="150" top="100" src="./assets/flashloan.png">

# flashloan-rs • [![ci](https://github.com/whitenois3/flashloan-rs/actions/workflows/tests.yaml/badge.svg)](https://github.com/whitenois3/flashloan-rs/actions/workflows/tests.yaml) ![license](https://img.shields.io/github/license/whitenois3/flashloan-rs) ![solidity](https://img.shields.io/badge/solidity-^0.8.15-lightgrey) ![Crates.io](https://img.shields.io/crates/v/flashloan-rs)

Minimal Multicall3 Flashloan Module.


### Getting Started

[Flashloan-rs](https://github.com/whitenois3/flashloan-rs) is published to crates.io as [flashloan-rs](https://crates.io/crates/flashloan-rs).

To use the crate in a Rust project, run the cargo add command like so: `cargo add flashloan-rs`.

Or, add the following to your Cargo.toml:

```toml
[dependencies]
flashloan-rs = "0.2.2"
```


### Usage

[Flashloan-rs](https://github.com/whitenois3/flashloan-rs) is built to be extremely simple to use.

**Quick Construction**

```rust,ignore
use std::{str::FromStr, sync::Arc};
use flashloan_rs::prelude::*;
use ethers::prelude::*;

// Create a web3 provider
let client = Provider::<Http>::try_from("https://eth-mainnet.g.alchemy.com/v2/your-api-key").unwrap();
let arc_client = Arc::new(client);

// Config
let wallet_address = Address::from_str("YOUR_ADDRESS").unwrap();
let lender = Address::from_str("LENDER_ADDRESS").unwrap();
let token_to_flashloan = Address::from_str("TOKEN_ADDRESS_TO_FLASHLOAN").unwrap();
let amount_to_flashloan = U256::from_dec_str("1000000000000000000").unwrap();

// Create a flashloan builder
// Alternatively, these parameters can be set using the builder pattern (see the next example)
let mut builder = FlashloanBuilder::new(
    Arc::clone(&arc_client),    // web3 provider
    1,                          // chain id
    Some(wallet_address),       // wallet public address
    Some(lender),               // address of the EIP-3156 Compliant Flash Lender
    Some(token_to_flashloan),   // token address to flashloan
    Some(amount_to_flashloan),  // amount to flashloan
    None,                       // override the flash borrower contract
);

// Deploy the flashloan borrower contract
builder.deploy(None, None).await.unwrap();

// Execute the flashloan and grab the transaction receipt
let optional_tx_receipt = builder.execute().await.unwrap();
let tx_receipt = optional_tx_receipt.unwrap();
```

**Builder Pattern**

```rust,ignore
use std::{str::FromStr, sync::Arc};
use flashloan_rs::prelude::*;
use ethers::prelude::*;

// Create a web3 provider
let client = Provider::<Http>::try_from("https://eth-mainnet.g.alchemy.com/v2/your-api-key").unwrap();
let arc_client = Arc::new(client);

// Config
let wallet_address = Address::from_str("YOUR_ADDRESS").unwrap();
let lender = Address::from_str("LENDER_ADDRESS").unwrap();
let token_to_flashloan = Address::from_str("TOKEN_ADDRESS_TO_FLASHLOAN").unwrap();
let amount_to_flashloan = U256::from_dec_str("1000000000000000000").unwrap();

// Create a flashloan builder
let mut builder = FlashloanBuilder::new(arc_client, 1, None, None, None, None, None);

// Set values using the builder pattern
builder.with_owner(wallet_address).with_lender(lender).with_token(token_to_flashloan).with_amount(amount_to_flashloan);

// ...
```


### Blueprint

```
flashloan-rs
├─ contracts
│  ├─ interfaces
│  │  ├─ IERC20.sol — ERC20 interface
│  │  ├─ IERC3156FlashBorrower.sol — Flashloan borrower interface
|  |  └─ IERC3156FlashLender.sol — Flashloan lender interface
│  ├─ FlashBorrower.huff — A https://github.com/huff-language Flashloan Receiver Contract Implementation
│  └─ FlashBorrower.sol — An Extensible Flashloan Receiver Contract
├─ examples
│  ├─ custom_borrower.rs — Flashloan-rs usage with a custom borrower contract
│  └─ pure_arb.rs — Executing a pure arbitrage with flashloan-rs
├─ lib — Foundry Libraries
├─ src
│  ├─ builder.rs — The primary rust FlashloanBuilder library
│  ├─ contract.rs — Abi Generated FlashBorrower Contract
│  ├─ errors.rs — Custom errors for flashloan-rs
│  └─ lib.rs — Module Exports
├─ tests
│  ├─ contracts
│  │  └─ FlashBorrower.t.sol — FlashBorrower.sol test suite
│  └─ crate
|     └─ builder.rs — Unit tests for flashloan-rs
├─ foundry.toml — Foundry Config
└─ Cargo.toml — The flashloan-rs Cargo Manifest
```


### License

[MIT](https://github.com/whitenois3/flashloan-rs/blob/main/LICENSE)


### Acknowledgements

A few very notable repositories that were used as reference:

- [multicall3](https://github.com/mds1/multicall)
- [ethers-rs](https://github.com/gakonst/ethers-rs)
- [yield-liquidator](https://github.com/yieldprotocol/yield-liquidator)
- [gelato-sdk](https://github.com/nomad-xyz/gelato-sdk)
