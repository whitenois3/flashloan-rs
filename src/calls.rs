
use ethers::prelude::*;
use serde::{Serialize, Deserialize};

use crate::errors::FlashloanError;

/// A Call3 Data Struct
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Call3 {
    /// The target address
    pub target: Address,
    /// Allow call failure
    #[serde(rename = "allowFailure" )]
    pub allow_failure: bool,
    /// A call value
    #[serde(serialize_with = "serialize_optional_u256")]
    pub value: Option<U256>,
    /// The call data
    #[serde(rename = "callData")]
    pub data: Bytes,
}

/// Serialize an Optional U256
pub fn serialize_optional_u256<S>(optional_uint: &Option<U256>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    optional_uint.unwrap_or_else(|| U256::zero()).to_string().serialize(s)
}


impl TryFrom<&str> for Call3 {
    type Error = FlashloanError<_>;

    fn try_from(string: &str) -> Result<Self, FlashloanError<_>> {
        serde_json::from_str::<Call3>(string).map_err(|e| FlashloanError::CallConstructionError(string.to_string()))
    }
}