#![feature(pointer_byte_offsets)]
#![feature(read_buf)]
#![feature(once_cell)]

use std::{
    fs,
    io::{self, Read, ReadBuf},
};

mod constant;
mod items;
pub use constant::limitation::*;
use constant::offset;
pub use items::*;

pub struct Profile(Vec<u8>);

impl Profile {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut profile = fs::OpenOptions::new().read(true).create(false).open(path)?;

        let mut buf = Vec::new();
        let mut buf_reader = ReadBuf::new(&mut buf);
        profile.read_buf_exact(&mut buf_reader)?;
        // currently skip file verfication, as it's not of importance

        Ok(Profile(buf.into()))
    }

    fn edit_int(&mut self, offset: isize, value: i32) {
        let p = self.0.as_mut_ptr() as *mut i32;
        unsafe {
            *p.byte_offset(offset) = value;
        }
    }

    pub fn save_map(&mut self, current_map: i32) {
        self.edit_int(offset::SAVE_LOCATION, current_map);
    }

    pub fn current_health(&mut self, current_health: i32) {
        self.edit_int(offset::CURRENT_HEALTH, current_health);
    }
}
