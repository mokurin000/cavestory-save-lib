pub(super) const POSITION_X: usize = 0x11;
pub(super) const POSITION_Y: usize = 0x15;

pub(super) const MAP: usize = 0x08;

pub(super) const MAX_HEALTH: usize = 0x1C;
pub(super) const HEALTH: usize = 0x20;

pub(super) const EQUIPPED_ITEMS: usize = 0x2C;

pub(super) const TELEPORTER_SIZE: usize = 0x08; // up to seven is safe
pub(super) const FIRST_TELEPORTER_MENU: usize = 0x158;
pub(super) const FIRST_TELEPORTER_LOCATION: usize = 0x15c; // 2bytes

pub(super) const WEAPON_SIZE: usize = 0x14;
pub(super) const FIRST_WEAPON_TYPE: usize = 0x38;
pub(super) const FIRST_WEAPON_LEVEL: usize = 0x3C;
pub(super) const FIRST_WEAPON_EXP: usize = 0x40;
pub(super) const FIRST_WEAPON_MAX_AMMO: usize = 0x44;
pub(super) const FIRST_WEAPON_AMMO: usize = 0x48;

pub(super) const INVENTORY_SIZE: usize = 0x04;
pub(super) const FIRST_INVENTORY: usize = 0xd8;

pub(super) const SONG: usize = 0x0C;
