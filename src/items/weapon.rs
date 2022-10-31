use std::mem::zeroed;

use strum::Display;
use strum::EnumIter;
use strum::FromRepr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weapon {
    /// type of this weapon
    pub classification: WeaponType,

    /// normally will be 0 to 3.
    pub level: i32,

    pub exp: i32,

    pub ammo: i32,

    /// zero means infinite ammos.
    pub max_ammo: i32,
}

impl Default for Weapon {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum WeaponType {
    None,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Snake)
    Snake,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Polar_Star)
    PolarStar,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Fireball)
    Fireball,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Machine_Gun)
    MachineGun,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Missile_Launcher)
    MissileLauncher,

    #[deprecated = "Bad weapon"]
    Missiles,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Bubbline)
    Bubbler,

    #[deprecated = "Bad weapon"]
    Unknown,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Blade)
    Blade,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Missile_Launcher#Super_Missile_Launcher)
    SuperMissile,

    #[deprecated = "Bad weapon"]
    SuperMissiles,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Nemesis)
    Nemesis,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Spur)
    Spur,

    #[deprecated = "Bad weapon"]
    Hajime,
}

impl From<i32> for WeaponType {
    fn from(v: i32) -> Self {
        WeaponType::from_repr(v as u32).unwrap_or(WeaponType::None)
    }
}
