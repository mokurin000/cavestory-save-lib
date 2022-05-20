#![feature(pointer_byte_offsets)]
#![feature(read_buf)]

use std::{
    fs,
    io::{self, Read, ReadBuf},
};

mod constants;
use constants::*;

pub struct Profile(Vec<u8>);

impl Profile {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut profile = fs::OpenOptions::new().read(true).create(false).open(path)?;

        let mut buf = Vec::new();
        let mut buf_reader = ReadBuf::new(&mut buf);
        profile.read_buf_exact(&mut buf_reader)?;

        Ok(Profile(buf.into()))
    }

    pub fn current_health(&mut self, current_health: i32) -> Result<(), io::Error> {
        let p = self.0.as_mut_ptr() as *mut i32;
        unsafe {
            *p.byte_offset(offset::SAVE_LOCATION) = current_health;
        }
        Ok(())
    }
}
