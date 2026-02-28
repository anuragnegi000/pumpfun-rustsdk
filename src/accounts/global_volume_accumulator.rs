use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: Pubkey,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}

pub const GLOBAL_VOLUME_ACCUMULATOR_DISCRIMINATOR: [u8; 8] = [202, 42, 246, 43, 142, 190, 30, 255];

impl GlobalVolumeAccumulator {
    pub const LEN: usize = 544;

    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        if data.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Data too short for discriminator",
            ));
        }
        let mut reader = &data[8..];
        Self::deserialize_reader(&mut reader)
    }
}
