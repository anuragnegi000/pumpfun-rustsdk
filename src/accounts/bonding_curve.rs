use borsh::BorshDeserialize;
use borsh::BorshSerialize;

pub const BONDING_CURVE_DISCRIMINATOR: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];

pub const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";
pub const GLOBAL_SEED: &[u8] = b"global";
pub const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";
pub const CREATOR_VAULT_SEED: &[u8] = b"creator-vault";
pub const GLOBAL_VOLUME_ACCUMULATOR_SEED: &[u8] = b"global_volume_accumulator";
pub const USER_VOLUME_ACCUMULATOR_SEED: &[u8] = b"user_volume_accumulator";
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";
pub const BONDING_CURVE_V2_SEED: &[u8] = b"bonding-curve-v2";

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
    pub is_cashback_coin: bool,
}

impl BondingCurve {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 1 + 32 + 1 + 1;

    pub fn safe_deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Data too short for discriminator",
            ));
        }
        let discriminator: [u8; 8] = data[0..8].try_into().unwrap();
        if discriminator != BONDING_CURVE_DISCRIMINATOR {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid discriminator: {:?}", discriminator),
            ));
        }
        let mut reader = &data[8..];
        Self::deserialize_reader(&mut reader)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        Self::safe_deserialize(data)
    }
}
