#![feature(pointer_byte_offsets)]
#![feature(once_cell)]

use std::{fs, io};

mod constant;
mod items;
pub use constant::limitation::*;
use constant::offset;
pub use items::*;

#[derive(Clone)]
pub struct Profile(Vec<u8>);

impl Profile {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        // currently skip file verfication, as it's not of importance
        Ok(Profile(fs::read(path)?))
    }

    fn edit_int(&mut self, offset: isize, value: i32) {
        let p = self.0.as_mut_ptr() as *mut i32;
        unsafe {
            *p.byte_offset(offset) = value;
        }
    }

    pub fn current_health(&mut self, current_health: i32) {
        self.edit_int(offset::CURRENT_HEALTH, current_health);
    }

    pub fn max_health(&mut self, max_health: i32) {
        self.edit_int(offset::MAX_HEALTH, max_health);
    }

    pub fn weapon_type(&mut self, weapon: i32, slot: isize) {
        self.edit_int(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot, weapon);
    }

    pub fn weapon_level(&mut self, level: i32, slot: isize) {
        self.edit_int(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot, level);
    }

    pub fn write_to(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, &self.0)?;
        Ok(())
    }
}
