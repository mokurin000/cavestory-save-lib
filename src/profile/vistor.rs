use super::Profile;
use super::offset::*;

impl Profile {
    pub fn set_position_x(&mut self, x: i16) {
        self.edit16(POSITION_X, x);
    }

    pub fn set_position_y(&mut self, y: i16) {
        self.edit16(POSITION_Y, y);
    }

    pub fn set_map(&mut self, map: i32) {
        self.edit(MAP, map);
    }

    pub fn set_health(&mut self, health: i16) {
        self.edit16(HEALTH, health);
    }

    pub fn set_max_health(&mut self, max_health: i16) {
        self.edit16(MAX_HEALTH, max_health);
    }

    pub fn set_weapon_type(&mut self, weapon: i32, slot: usize) {
        self.edit(FIRST_WEAPON_TYPE + WEAPON_SIZE * slot, weapon);
    }

    pub fn set_weapon_level(&mut self, level: i32, slot: usize) {
        self.edit(FIRST_WEAPON_LEVEL + WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_exp(&mut self, level: i32, slot: usize) {
        self.edit(FIRST_WEAPON_EXP + WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_ammo(&mut self, ammo: i32, slot: usize) {
        self.edit(FIRST_WEAPON_AMMO + WEAPON_SIZE * slot, ammo);
    }

    pub fn set_weapon_max_ammo(&mut self, max_ammo: i32, slot: usize) {
        self.edit(
            FIRST_WEAPON_MAX_AMMO + WEAPON_SIZE * slot,
            max_ammo,
        );
    }

    pub fn set_inventory(&mut self, inventory: i32, slot: usize) {
        self.edit(FIRST_INVENTORY_TYPE + INVENTORY_SIZE * slot, inventory)
    }

    pub fn set_music(&mut self, song: i32) {
        self.edit(SONG, song);
    }

    pub fn position_x(&self) -> i16 {
        self.read16(POSITION_X)
    }

    pub fn position_y(&self) -> i16 {
        self.read16(POSITION_Y)
    }

    pub fn map(&self) -> i32 {
        self.read(MAP)
    }

    pub fn health(&self) -> i16 {
        self.read16(HEALTH)
    }

    pub fn max_health(&self) -> i16 {
        self.read16(MAX_HEALTH)
    }

    pub fn weapon_type(&self, slot: usize) -> i32 {
        self.read(FIRST_WEAPON_TYPE + WEAPON_SIZE * slot)
    }

    pub fn weapon_level(&self, slot: usize) -> i32 {
        self.read(FIRST_WEAPON_LEVEL + WEAPON_SIZE * slot)
    }

    pub fn weapon_exp(&self, slot: usize) -> i32 {
        self.read(FIRST_WEAPON_EXP + WEAPON_SIZE * slot)
    }

    pub fn weapon_ammo(&self, slot: usize) -> i32 {
        self.read(FIRST_WEAPON_AMMO + WEAPON_SIZE * slot)
    }

    pub fn weapon_max_ammo(&self, slot: usize) -> i32 {
        self.read(FIRST_WEAPON_MAX_AMMO + WEAPON_SIZE * slot)
    }

    pub fn inventory(&self, slot: usize) -> i32 {
        self.read(FIRST_INVENTORY_TYPE + INVENTORY_SIZE * slot)
    }

    pub fn music(&self) -> i32 {
        self.read(SONG)
    }
}