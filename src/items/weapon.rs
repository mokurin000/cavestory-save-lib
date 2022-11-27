use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "PascalCase")]
#[repr(u32)]
pub enum WeaponType {
    #[default]
    None,
    Snake,
    PolarStar,
    Fireball,
    MachineGun,
    MissileLauncher,
    #[strum(serialize = "Missiles [bad]")]
    Missiles,
    Bubbler,
    #[strum(serialize = "Unknown [bad]")]
    Unknown,
    Blade,
    SuperMissile,
    #[strum(serialize = "SuperMissiles [bad]")]
    SuperMissiles,
    Nemesis,
    Spur,
    #[strum(serialize = "Hajime [bad]")]
    Hajime,
}

impl From<i32> for WeaponType {
    fn from(v: i32) -> Self {
        WeaponType::from_repr(v as u32).unwrap_or(Default::default())
    }
}
