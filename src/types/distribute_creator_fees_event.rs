use crate::types::Shareholder;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DistributeCreatorFeesEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sharing_config: Pubkey,
    pub admin: Pubkey,
    pub shareholders: Vec<Shareholder>,
    pub distributed: u64,
}
