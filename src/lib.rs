#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

/// Flashloan-rs errors
pub mod errors;

/// Main Flashloan Builder Module
pub mod builder;

/// Call3 Module
pub mod calls;

/// Re-export a prelude
pub mod prelude {
    pub use super::{
        builder::*,
        calls::*
    };
}
