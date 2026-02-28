use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetCreatorEvent {
    pub timestamp: i64,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator: Pubkey,
}
