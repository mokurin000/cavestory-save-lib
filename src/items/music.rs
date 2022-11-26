use strum::Display;
use strum::EnumIter;
use strum::FromRepr;

/// [Fandom Wiki](https://cavestory.fandom.com/wiki/Soundtrack)
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum Song {
    #[default]
    Nothing,
    MischievousRobot,
    Safety,
    GameOver,
    Gravity,
    OntoGrasstown,
    Meltdown2,
    EyesofFlame,
    Gestation,
    MimigaVillage,
    GetItem,
    #[strum(serialize = "Barlog's Theme")]
    BalrogsTheme,
    Cemetary,
    Plant,
    Pulse,
    BossDefeated,
    GetLifeCapsule,
    Tyrant,
    Run,
    Jenka1,
    LabyrinthFight,
    Access,
    Oppression,
    Geothermal,
    CaveStory,
    Moonsong,
    #[strum(serialize = "Hero's End")]
    HerosEnd,
    ScorchingBack,
    Quiet,
    FinalCave,
    Balcony,
    Charge,
    LastBattle,
    TheWayBackHome,
    Zombie,
    BreakDown,
    RunningHell,
    Jenka2,
    LivingWaterway,
    SealChamber,
    #[strum(serialize = "Toroko's Theme")]
    TorokosTheme,
    #[strum(serialize = "King's Theme")]
    KingsTheme,
}

impl From<i32> for Song {
    fn from(v: i32) -> Self {
        Song::from_repr(v as u32).unwrap_or(Default::default())
    }
}
