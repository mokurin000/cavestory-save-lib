mod equipment;
mod inventory;
mod map;
mod music;
mod teleporter;
mod weapon;

pub use equipment::Equip;
pub use equipment::Equipment;

pub use weapon::Weapon;
pub use weapon::WeaponType;

pub use inventory::Inventory;

pub use music::Song;

pub use map::Map;

pub use teleporter::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}
