#![feature(pointer_byte_offsets)]
#![feature(once_cell)]

use std::{fs, io};

mod constant;
mod items;
use constant::offset;
pub use items::*;

#[derive(Clone)]
pub struct Profile(Vec<u8>);

impl Profile {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        Ok(Profile(fs::read(path)?))
    }

    fn edit_int(&mut self, offset: isize, value: i32) {
        let p = self.0.as_mut_ptr() as *mut i32;
        unsafe {
            *p.byte_offset(offset) = value;
        }
    }

    fn read_int(&self, offset: isize) -> i32 {
        unsafe { *(self.0.as_ptr() as *const i32).byte_offset(offset) }
    }

    pub fn set_current_health(&mut self, current_health: i32) {
        self.edit_int(offset::CURRENT_HEALTH, current_health);
    }

    pub fn set_max_health(&mut self, max_health: i32) {
        self.edit_int(offset::MAX_HEALTH, max_health);
    }

    pub fn set_weapon_type(&mut self, weapon: i32, slot: isize) {
        self.edit_int(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot, weapon);
    }

    pub fn set_weapon_level(&mut self, level: i32, slot: isize) {
        self.edit_int(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_exp(&mut self, level: i32, slot: isize) {
        self.edit_int(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_ammo(&mut self, ammo: i32, slot: isize) {
        self.edit_int(
            offset::WEAPON_CURRENT_AMMO + offset::WEAPON_SIZE * slot,
            ammo,
        );
    }

    pub fn set_weapon_max_ammo(&mut self, max_ammo: i32, slot: isize) {
        self.edit_int(
            offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot,
            max_ammo,
        );
    }

    pub fn current_health(&self) -> i32 {
        self.read_int(offset::CURRENT_HEALTH)
    }

    pub fn max_health(&self) -> i32 {
        self.read_int(offset::MAX_HEALTH)
    }

    pub fn weapon_type(&self, slot: isize) -> i32 {
        self.read_int(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_level(&self, slot: isize) -> i32 {
        self.read_int(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_exp(&self, slot: isize) -> i32 {
        self.read_int(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_ammo(&self, slot: isize) -> i32 {
        self.read_int(offset::WEAPON_CURRENT_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_max_ammo(&self, slot: isize) -> i32 {
        self.read_int(offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn write_to(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, &self.0)?;
        Ok(())
    }
}
