mod offset;
mod vistor;

/// entire data from `profile.dat`
#[derive(Clone, Debug)]
pub struct Profile(Vec<u8>);

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

    pub fn verify(&self) -> bool {
        let p = self.0.as_ptr() as *const [u8; 8];
        unsafe {
            let head: [u8; 8] = p.read();
            &head == b"Do041220"
        }
    }
}

impl From<Vec<u8>> for Profile {
    fn from(content: Vec<u8>) -> Self {
        Profile(content)
    }
}

impl From<Profile> for Vec<u8> {
    fn from(profile: Profile) -> Self {
        profile.0
    }
}
