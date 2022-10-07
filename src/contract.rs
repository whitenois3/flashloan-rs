//! Refactor abi generation to allow missing docs

#![allow(missing_docs)]

use ethers::prelude::*;

abigen!(Flashloan, "src/FlashBorrower.json");
