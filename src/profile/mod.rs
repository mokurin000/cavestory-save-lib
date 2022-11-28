mod offset;
mod vistor;

use thiserror::Error;

/// entire data from `profile.dat`
#[derive(Clone, Debug)]
pub struct Profile(Vec<u8>);

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("file head not equal to \"Do041220\"!")]
    IllegalFileHead,

    #[error("file length not match!")]
    LengthNotMatch,
}

impl Profile {
    fn edit32(&mut self, offset: usize, value: i32) {
        let p = self.0.as_mut_ptr() as *mut i32;

        unsafe {
            *p.byte_offset(offset as isize) = if cfg!(target_endian = "little") {
                value
            } else {
                value.swap_bytes()
            }
        };
    }

    fn edit16(&mut self, offset: usize, value: i16) {
        let p = self.0.as_mut_ptr() as *mut i16;

        unsafe {
            *p.byte_offset(offset as isize) = if cfg!(target_endian = "little") {
                value
            } else {
                value.swap_bytes()
            }
        };
    }

    fn read32(&self, offset: usize) -> i32 {
        let native: i32 = unsafe { *(self.0.as_ptr() as *const i32).byte_offset(offset as isize) };
        if cfg!(target_endian = "little") {
            native
        } else {
            native.swap_bytes()
        }
    }

    fn read16(&self, offset: usize) -> i16 {
        let native: i16 = unsafe { *(self.0.as_ptr() as *const i16).byte_offset(offset as isize) };
        if cfg!(target_endian = "little") {
            native
        } else {
            native.swap_bytes()
        }
    }

    fn verify(&self) -> bool {
        let p = self.0.as_ptr() as *const [u8; 8];
        unsafe {
            let head: [u8; 8] = p.read();
            &head == b"Do041220"
        }
    }
}

impl TryFrom<Vec<u8>> for Profile {
    type Error = ProfileError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let profile = Profile(value);
        match (profile.0.len() == 0x603, profile.verify()) {
            (true, true) => Ok(profile),
            (false, _) => Err(ProfileError::LengthNotMatch),
            (_, false) => Err(ProfileError::IllegalFileHead),
        }
    }
}

impl From<Profile> for Vec<u8> {
    fn from(profile: Profile) -> Self {
        profile.0
    }
}
