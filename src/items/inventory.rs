use std::mem::transmute;
use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum Inventory {
    None,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Arthur's_Key)
    #[strum(serialize = "Arthur's Key")]
    ArthursKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Map_System)
    MapSystem,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Santa's_Key)
    #[strum(serialize = "Santa's Key")]
    SantasKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Silver_Locket)
    SilverLocket,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Beast_Fang)
    BeastFang,
    
    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Life_Capsule)
    LifeCapsule,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/ID_Card)
    IDCard,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Jellyfish_Juice)
    JellyfishJuice,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Rusted_Key)
    RustedKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Gum_Key)
    GumKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Gum_Base)
    GumBase,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Charcoal)
    Charcoal,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Bomb)
    Bomb,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Puppies)
    Dog,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Life_Pot)
    LifePot,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Cure-All)
    CureAll,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Clinic_Key)
    ClinicKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Booster_v0.8)
    #[strum(serialize = "Booster v0.8")]
    Boosterv08,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Arms_Barrier)
    ArmsBarrier,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Turbocharge)
    Turbocharge,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Curly's_Air_Tank)
    #[strum(serialize = "Curly's Air Tank")]
    AirTank,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/290_Counter)
    Counter290,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Booster_v2.0)
    Boosterv20,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Mimiga_Mask)
    MimigaMask,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Teleporter_Room_key)
    TeleporterRoomKey,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Sue's_Letter)
    #[strum(serialize = "Sue's Letter")]
    SuesLetter,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Controller)
    Controller,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Sprinklers#Broken_Sprinkler)
    BrokenSprinkler,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Sprinklers#Sprinkler)
    Sprinkler,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Tow_Rope)
    TowRope,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Medals#Medal_of_the_Red_Ogre)
    #[strum(serialize = "Medal of the Red Ogre")]
    MedaloftheRedOgre,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Littles#Tasks)
    MisterLittle,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Mushroom_Badge)
    MushroomBadge,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Ma_Pignon)
    MaPignon,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Curly's_Panties)
    #[strum(serialize = "Curly's Panties")]
    CurlysPanties,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Medals#Alien_Medal)
    AlienMedal,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Chako#Rouge/Lipstick)
    ChakosRouge,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Whimsical_Star)
    WhimsicalStar,

    /// [Fandom Wiki](https://cavestory.fandom.com/wiki/Iron_Bond)
    IronBond,
}

impl From<i32> for Inventory {
    fn from(v: i32) -> Self {
        unsafe { transmute(v) }
    }
}
