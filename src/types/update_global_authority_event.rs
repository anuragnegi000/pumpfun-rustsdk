use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateGlobalAuthorityEvent {
    pub global: Pubkey,
    pub authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}
