pub mod offset {
    pub const SAVE_LOCATION: isize = 0x08;
    pub const CURRENT_HEALTH: isize = 0x20;
    pub const MAX_HEALTH: isize = 0x1C;
    pub const WEAPON_SIZE: isize = 0x14;
    pub const WEAPON_TYPE: isize = 0x38;
    pub const WEAPON_LEVEL: isize = 0x3C;
    pub const WEAPON_EXP: isize = 0x40;
    pub const WEAPON_MAX_AMMO: isize = 0x44;
    pub const WEAPON_CURRENT_AMMO: isize = 0x48;
    pub const INVENTORY_TYPE: isize = 0xd8;
    pub const DIFFICULTY: isize = 0x610; // Cave Story+, 0 for normal, 2 for easy, 4 for hard
}

pub mod limitation {
    pub const INVENTORY_SLOT: i32 = 24; // each inventory is of 0x4 bytes
    pub const WEAPON_SLOT: i32 = 6;
}
