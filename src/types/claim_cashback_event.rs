use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimCashbackEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed: u64,
    pub total_cashback_earned: u64,
}
