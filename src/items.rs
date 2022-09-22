/*!
index = item id in savedata.
*/

use std::mem::{transmute, zeroed};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weapon {
    pub classification: WeaponType,

    pub level: i32,
    pub exp: i32,
    pub ammo: i32,
    pub max_ammo: i32,
}

impl Default for Weapon {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WeaponType {
    None,
    Snake,
    PolarStar,
    Fireball,
    MachineGun,
    MissileLauncher,
    #[deprecated = "Bad weapon"]
    Missiles,
    Bubbler,
    #[deprecated = "Bad weapon"]
    Unknown,
    Blade,
    SuperMissile,
    #[deprecated = "Bad weapon"]
    SuperMissiles,
    Nemesis,
    Spur,
    #[deprecated = "Bad weapon"]
    Hajime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Inventory {
    None,
    ArthursKey,
    MapSystem,
    SantasKey,
    SilverLocket,
    BeastFang,
    LifeCapsule,
    IDCard,
    JellyfishJuice,
    RustedKey,
    GumKey,
    GumBase,
    Charcoal,
    Bomb,
    Dog,
    LifePot,
    CureAll,
    ClinicKey,
    Boosterv08,
    ArmsBarrier,
    Turbocharge,
    AirTank,
    Counter290,
    Boosterv20,
    MimigaMask,
    TeleporterRoomKey,
    SuesLetter,
    Controller,
    BrokenSprinkler,
    Sprinkler,
    TowRope,
    MedaloftheRedOgre,
    MisterLittle,
    MushroomBadge,
    MaPignon,
    CurlysPanties,
    AlienMedal,
    ChakosRouge,
    WhimsicalStar,
    IronBond,
}

impl From<i32> for Inventory {
    fn from(v: i32) -> Self {
        unsafe { transmute(v) }
    }
}

impl From<i32> for WeaponType {
    fn from(v: i32) -> Self {
        unsafe { transmute(v) }
    }
}
