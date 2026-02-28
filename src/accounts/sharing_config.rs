use crate::types::ConfigStatus;
use crate::types::Shareholder;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

pub const SHARING_CONFIG_DISCRIMINATOR: [u8; 8] = [216, 74, 9, 0, 56, 140, 93, 75];

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharingConfig {
    pub bump: u8,
    pub version: u8,
    pub status: ConfigStatus,
    pub mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub admin_revoked: bool,
    pub shareholders: Vec<Shareholder>,
}

impl SharingConfig {
    pub fn safe_deserialize(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Data too short for discriminator",
            ));
        }
        let discriminator: [u8; 8] = data[0..8].try_into().unwrap();
        if discriminator != SHARING_CONFIG_DISCRIMINATOR {
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
