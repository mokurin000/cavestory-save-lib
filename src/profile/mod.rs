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
        if offset + 4 > self.0.len() {
            panic!("out of bound");
        }

        let p = self.0.as_mut_ptr() as *mut [u8; 4];

        unsafe {
            p.byte_offset(offset as isize)
                .write_unaligned(value.to_le_bytes())
        };
    }

    fn edit16(&mut self, offset: usize, value: i16) {
        if offset + 2 > self.0.len() {
            panic!("out of bound");
        }

        let p = self.0.as_mut_ptr() as *mut [u8; 2];

        unsafe {
            p.byte_offset(offset as isize)
                .write_unaligned(value.to_le_bytes())
        };
    }

    fn read32(&self, offset: usize) -> i32 {
        if offset + 4 > self.0.len() {
            panic!("out of bound");
        }

        let bytes: [u8; 4] = unsafe {
            (self.0.as_ptr() as *const [u8; 4])
                .byte_offset(offset as _)
                .read_unaligned()
        };
        i32::from_le_bytes(bytes)
    }

    fn read16(&self, offset: usize) -> i16 {
        if offset + 2 > self.0.len() {
            panic!("out of bound");
        }

        let bytes: [u8; 2] = unsafe {
            (self.0.as_ptr() as *const [u8; 2])
                .byte_offset(offset as _)
                .read_unaligned()
        };
        i16::from_le_bytes(bytes)
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

        if profile.0.len() != 0x604 {
            return Err(ProfileError::LengthNotMatch);
        }
        if !profile.verify() {
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
