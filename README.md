<img align="right" width="150" height="150" top="100" src="./assets/flashloan.png">

# flashloan-rs • [![ci](https://github.com/whitenois3/flashloan-rs/actions/workflows/tests.yaml/badge.svg)](https://github.com/whitenois3/flashloan-rs/actions/workflows/tests.yaml) ![license](https://img.shields.io/github/license/whitenois3/flashloan-rs) ![solidity](https://img.shields.io/badge/solidity-^0.8.15-lightgrey) ![Crates.io](https://img.shields.io/crates/v/flashloan-rs)

Minimal Multicall3 Flashloan Module.


### Getting Started

[Flashloan-rs](https://github.com/whitenois3/flashloan-rs) is published to crates.io as [flashloan-rs](https://crates.io/crates/flashloan-rs).

To use the crate in a Rust project, run the cargo add command like so: `cargo add flashloan-rs`.

Or, add the following to your Cargo.toml:

```toml
[dependencies]
flashloan-rs = "0.1.0"
```


### Usage

[Flashloan-rs](https://github.com/whitenois3/flashloan-rs) is built to be extremely simple to use.


```rust
// TODO
```



### Blueprint

```ml
.
├─ contracts
│  ├─ interfaces
│  │  ├─ IERC20.sol — ERC20 interface
│  │  ├─ IERC3156FlashBorrower.sol — Flashloan borrower interface
|  |  └─ IERC3156FlashLender.sol — Flashloan lender interface
│  ├─ FlashBorrower.huff — A [huff](https://github.com/huff-language) Flashloan Receiver Contract Implementation
│  └─ FlashBorrower.sol — An Extensible Flashloan Receiver Contract
├─ examples
│  ├─ custom_borrower.rs — Flashloan-rs usage with a custom borrower contract
│  └─ pure_arb.rs — Executing a pure arbitrage with flashloan-rs
├─ lib — Foundry Libraries
├─ src
│  ├─ builder.rs — The FlashloanBuilder
│  ├─ errors.rs — Custom errors for flashloan-rs
│  └─ lib.rs — Module Exports
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
