use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdminSetCreatorEvent {
    pub timestamp: i64,
    pub admin_set_creator_authority: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub old_creator: Pubkey,
    pub new_creator: Pubkey,
}
