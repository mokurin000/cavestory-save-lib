#![feature(pointer_byte_offsets)]

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

    fn edit<T: Copy>(&mut self, offset: isize, value: T) {
        let p = self.0.as_mut_ptr() as *mut T;
        unsafe {
            *p.byte_offset(offset) = value;
        }
    }

    fn read<T: Copy>(&self, offset: isize) -> T {
        unsafe { *(self.0.as_ptr() as *const T).byte_offset(offset) }
    }

    pub fn set_health(&mut self, health: i16) {
        self.edit(offset::HEALTH, health);
    }

    pub fn set_max_health(&mut self, max_health: i16) {
        self.edit(offset::MAX_HEALTH, max_health);
    }

    pub fn set_weapon_type(&mut self, weapon: i32, slot: isize) {
        self.edit(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot, weapon);
    }

    pub fn set_weapon_level(&mut self, level: i32, slot: isize) {
        self.edit(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_exp(&mut self, level: i32, slot: isize) {
        self.edit(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_ammo(&mut self, ammo: i32, slot: isize) {
        self.edit(offset::WEAPON_AMMO + offset::WEAPON_SIZE * slot, ammo,
        );
    }

    pub fn set_weapon_max_ammo(&mut self, max_ammo: i32, slot: isize) {
        self.edit(offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot, max_ammo);
    }

    pub fn set_inventory(&mut self, inventory: i32, slot: isize) {
        self.edit(offset::INVENTORY_TYPE + 4*slot, inventory)
    }

    pub fn health(&self) -> i16 {
        self.read(offset::HEALTH)
    }

    pub fn max_health(&self) -> i16 {
        self.read(offset::MAX_HEALTH)
    }

    pub fn weapon_type(&self, slot: isize) -> i32 {
        self.read(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_level(&self, slot: isize) -> i32 {
        self.read(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_exp(&self, slot: isize) -> i32 {
        self.read(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_ammo(&self, slot: isize) -> i32 {
        self.read(offset::WEAPON_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_max_ammo(&self, slot: isize) -> i32 {
        self.read(offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn inventory(&self, slot:isize) -> i32 {
        self.read(offset::INVENTORY_TYPE + 4*slot)
    }

    pub fn write_to(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, &self.0)?;
        Ok(())
    }
}
