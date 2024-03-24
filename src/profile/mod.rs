mod offset;
mod vistor;

use thiserror::Error;

/// entire data from `profile.dat`
#[derive(Clone, Debug)]
pub struct Profile(Vec<u8>);

#[derive(Error, Debug, Clone, Copy)]
pub enum ProfileError {
    #[error("file head not equal to \"Do041220\"!")]
    IllegalFileHead,

    #[error("file length not match!")]
    LengthNotMatch,
}

impl Profile {
    fn edit32(&mut self, offset: usize, value: i32) {
        self.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes())
    }

    fn edit16(&mut self, offset: usize, value: i16) {
        self.0[offset..offset + 2].copy_from_slice(&value.to_le_bytes())
    }

    fn read32(&self, offset: usize) -> i32 {
        let mut bytes: [u8; 4] = Default::default();
        bytes.copy_from_slice(&self.0[offset..offset + 4]);
        i32::from_le_bytes(bytes)
    }

    fn read16(&self, offset: usize) -> i16 {
        let mut bytes: [u8; 2] = Default::default();
        bytes.copy_from_slice(&self.0[offset..offset + 2]);
        i16::from_le_bytes(bytes)
    }

    fn verify_header(&self) -> bool {
        let head = &self.0[..8];
        &head == b"Do041220"
    }

    pub fn from_raw_without_length_check(data: Vec<u8>) -> Result<Self, ProfileError> {
        let profile = Profile(data);

        if profile.0.len() < 0x604 {
            return Err(ProfileError::LengthNotMatch);
        }
        if !profile.verify_header() {
            return Err(ProfileError::IllegalFileHead);
        }

        Ok(profile)
    }

    pub fn from_raw_unchecked(data: Vec<u8>) -> Self {
        Profile(data)
    }
}

impl TryFrom<Vec<u8>> for Profile {
    type Error = ProfileError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let profile = Profile(value);

        if profile.0.len() != 0x604 {
            return Err(ProfileError::LengthNotMatch);
        }
        if !profile.verify_header() {
            return Err(ProfileError::IllegalFileHead);
        }

        Ok(profile)
    }
}

impl From<Profile> for Vec<u8> {
    fn from(profile: Profile) -> Self {
        profile.0
    }
}
