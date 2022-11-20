mod weapon;
mod inventory;
mod music;
mod map;

pub use weapon::Weapon;
pub use weapon::WeaponType;

pub use inventory::Inventory;

pub use music::Song;

pub use map::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}