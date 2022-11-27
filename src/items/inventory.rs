use strum::{Display, EnumIter, FromRepr};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum Inventory {
    #[default]
    None,

    #[strum(serialize = "Arthur's Key")]
    ArthursKey,

    MapSystem,

    #[strum(serialize = "Santa's Key")]
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

    #[strum(serialize = "Booster v0.8")]
    Boosterv08,

    ArmsBarrier,

    Turbocharge,

    #[strum(serialize = "Curly's Air Tank")]
    AirTank,

    #[strum(serialize = "290 Counter")]
    Counter290,

    #[strum(serialize = "Booster v2.0")]
    Boosterv20,

    MimigaMask,

    TeleporterRoomKey,

    #[strum(serialize = "Sue's Letter")]
    SuesLetter,

    Controller,

    BrokenSprinkler,

    Sprinkler,

    TowRope,

    #[strum(serialize = "Medal of the Red Ogre")]
    MedaloftheRedOgre,

    MisterLittle,

    MushroomBadge,

    MaPignon,

    #[strum(serialize = "Curly's Panties")]
    CurlysPanties,

    AlienMedal,

    ChakosRouge,

    WhimsicalStar,

    IronBond,
}

impl From<i32> for Inventory {
    fn from(v: i32) -> Self {
        Inventory::from_repr(v as u32).unwrap_or(Default::default())
    }
}
