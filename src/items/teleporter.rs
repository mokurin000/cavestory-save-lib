use strum::Display;
use strum::EnumIter;
use strum::FromRepr;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Teleporter {
    pub menu: TeleporterMenu,
    pub location: TeleporterLocation,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum TeleporterMenu {
    #[default]
    Nothing,
    EggCorridor,
    Grasstown,
    SandZone,
    Labyrinth,
    Plantation,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u16)]
pub enum TeleporterLocation {
    #[default]
    EggCorridor = 0x17_71,
    Grasstown,
    SandZone,
    Labyrinth,
    Plantation,
}

impl From<i32> for TeleporterMenu {
    fn from(v: i32) -> Self {
        TeleporterMenu::from_repr(v as u32).unwrap_or(Default::default())
    }
}

impl From<i16> for TeleporterLocation {
    fn from(v: i16) -> Self {
        TeleporterLocation::from_repr(v as u16).unwrap_or(TeleporterLocation::EggCorridor)
    }
}
