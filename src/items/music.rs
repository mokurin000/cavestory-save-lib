use std::mem::transmute;

/// [Fandom Wiki](https://cavestory.fandom.com/wiki/Soundtrack)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Song {
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
    TorokosTheme,
    KingsTheme,
}

impl From<i32> for Song {
    fn from(v: i32) -> Self {
        unsafe { transmute(v) }
    }
}
