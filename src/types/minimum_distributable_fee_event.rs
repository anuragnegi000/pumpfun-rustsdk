use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MinimumDistributableFeeEvent {
    pub minimum_required: u64,
    pub distributable_fees: u64,
    pub can_distribute: bool,
}
