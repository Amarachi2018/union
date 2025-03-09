use unionlabs::primitives::H256;

use crate::custom_types::Version;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ssz", derive(ssz::Ssz))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: H256,
}
