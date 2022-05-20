#![feature(pointer_byte_offsets)]
#![feature(read_buf)]
#![feature(once_cell)]

use std::{
    fs,
    io::{self, BufWriter, Read, Write},
};

mod constant;
mod items;
pub use constant::limitation::*;
use constant::offset;
pub use items::*;

#[derive(Clone)]
pub struct Profile(Vec<u8>);

impl Profile {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut profile = fs::OpenOptions::new().read(true).create(false).open(path)?;

        let length = profile.metadata()?.len();
        let mut buf = Vec::with_capacity(length as usize);
        profile.read(&mut buf)?;
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

    pub fn max_health(&mut self, max_health: i32) {
        self.edit_int(offset::MAX_HEALTH, max_health);
    }

    pub fn write_to(mut self, path: &str) -> Result<(), io::Error> {
        let mut profile = fs::OpenOptions::new().write(true).create(true).open(path)?;
        let mut buf = BufWriter::new(&mut profile);
        buf.write_all(&mut self.0)?;
        Ok(())
    }
}
