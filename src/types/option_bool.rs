use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OptionBool(pub bool);

impl Default for OptionBool {
    fn default() -> Self {
        Self(false)
    }
}

impl OptionBool {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn is_true(&self) -> bool {
        self.0
    }
}

impl From<bool> for OptionBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
