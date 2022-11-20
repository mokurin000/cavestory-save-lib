use strum::Display;
use strum::EnumIter;
use strum::FromRepr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
#[repr(u32)]
pub enum Map {
    #[strum(serialize = "Credits")]
    Credits,
    #[strum(serialize = "Arthur's House - normal")]
    ArthursHouse,
    #[strum(serialize = "Egg Corridor")]
    EggCorridor,
    #[strum(serialize = "Egg No. 00 - normal")]
    EggNo00,
    #[strum(serialize = "Egg No. 06")]
    EggNo06,
    #[strum(serialize = "Egg Observation Room")]
    EggObservationRoom,
    #[strum(serialize = "Grasstown")]
    Grasstown,
    #[strum(serialize = "Santa's House")]
    SantasHouse,
    #[strum(serialize = "Chaco's House")]
    ChacosHouse,
    #[strum(serialize = "Labyrinth I - ertical starting room")]
    LabyrinthIErticalStartingRoom,
    #[strum(serialize = "Sand Zone - normal")]
    SandZone,
    #[strum(serialize = "Mimiga Village")]
    MimigaVillage,
    #[strum(serialize = "First Cave")]
    FirstCave,
    #[strum(serialize = "Start Point")]
    StartPoint,
    #[strum(serialize = "Shack (Mimiga Village)")]
    ShackMimigaVillage,
    #[strum(serialize = "Reservoir ")]
    Reservoir,
    #[strum(serialize = "Graveyard ")]
    Graveyard,
    #[strum(serialize = "Yamashita Farm ")]
    YamashitaFarm,
    #[strum(serialize = "Shelter (Grasstown)")]
    Shelter,
    #[strum(serialize = "Assembly Hall (Mimiga Village)")]
    AssemblyHall,
    #[strum(serialize = "Save Point (Mimiga Village)")]
    SavePointAtMimigaVillage,
    #[strum(serialize = "Side Room (Egg Corridor)")]
    SideRoom,
    #[strum(serialize = "Cthulhu's Abode (Egg Corridor)")]
    CthulhusAbode,
    #[strum(serialize = "Egg No. 01")]
    EggNo01,
    #[strum(serialize = "Arthur's House - sue using computer")]
    ArthursHouseWithSueUsingComputer,
    #[strum(serialize = "Power Room (Grasstown)")]
    PowerRoom,
    #[strum(serialize = "Save Point (Grasstown)")]
    SavePointAtGrasstown,
    #[strum(serialize = "Execution Chamber (Grasstown)")]
    ExecutionChamber,
    #[strum(serialize = "Gum (Grasstown)")]
    Gum,
    #[strum(serialize = "Sand Zone Residence")]
    SandZoneResidence,
    #[strum(serialize = "Grasstown Hut")]
    GrasstownHut,
    #[strum(serialize = "Main Artery (Waterway)")]
    MainArtery,
    #[strum(serialize = "Small Room (Sand Zone)")]
    SmallRoom,
    #[strum(serialize = "Jenka's House - before Balrog attack")]
    JenkasHouse,
    #[strum(serialize = "Deserted House (Sand Zone)")]
    DesertedHouse,
    #[strum(serialize = "Sand Zone Storehouse")]
    SandZoneStorehouse,
    #[strum(serialize = "Jenka's House - after Balrog attack")]
    JenkasHouseAfterBalrogAttack,
    #[strum(serialize = "Sand Zone - after boss fight")]
    SandZoneAfterBossFight,
    #[strum(serialize = "Labyrinth H - sliding block room")]
    LabyrinthH,
    #[strum(serialize = "LabyrinthW_mainareawithshop,camp")]
    LabyrinthWMainAreaWithShopCamp,
    #[strum(serialize = "Camp (Labyrinth)")]
    Camp,
    #[strum(serialize = "Clinic Ruins (Labyrinth)")]
    ClinicRuins,
    #[strum(serialize = "Labyrinth Shop")]
    LabyrinthShop,
    #[strum(serialize = "Labyrinth B - room with booster")]
    LabyrinthBroomwithbooster,
    #[strum(serialize = "Boulder Chamber (Labyrinth)")]
    BoulderChamber,
    #[strum(serialize = "LabyrinthM_lastareagaudieggs")]
    LabyrinthMLastAreaGaudiEggs,
    #[strum(serialize = "Dark Place (Labyrinth)")]
    DarkPlace,
    #[strum(serialize = "Core (Labyrinth)")]
    Core,
    #[strum(serialize = "Waterway")]
    Waterway,
    #[strum(serialize = "Egg Corridor?")]
    QuestionableEggCorridor,
    #[strum(serialize = "Cthulhu's Abode? (Egg Corridor?)")]
    QuestionableCthulhusAbode,
    #[strum(serialize = "Egg Observation Room?")]
    QuestionableEggObservationRoom,
    #[strum(serialize = "Egg No. 00 - hatched")]
    EggNo00WhenHatched,
    #[strum(serialize = "Outer Wall")]
    OuterWall,
    #[strum(serialize = "Side Room (Egg Corridor?)")]
    QuestionableSideRoom,
    #[strum(serialize = "Storehouse (Outer Wall)")]
    Storehouse,
    #[strum(serialize = "Plantation")]
    Plantation,
    #[strum(serialize = "Jail No. 1 (Plantation)")]
    JailNo1,
    #[strum(serialize = "Hideout (Plantation)")]
    Hideout,
    #[strum(serialize = "Rest Area (Plantation)")]
    RestArea,
    #[strum(serialize = "Teleporter (Plantation)")]
    Teleporter,
    #[strum(serialize = "Jail No. 2 (Plantation)")]
    JailNo2,
    #[strum(serialize = "Balcony - normal")]
    Balcony,
    #[strum(serialize = "Last Cave")]
    LastCave,
    #[strum(serialize = "Throne Room (Balcony)")]
    ThroneRoom,
    #[strum(serialize = "The King's Table (Balcony)")]
    TheKingsTable,
    #[strum(serialize = "Prefab House (Balcony) - normal")]
    PrefabHouse,
    #[strum(serialize = "Last Cave Hidden")]
    LastCaveHidden,
    #[strum(serialize = "Black Space (Balcony)")]
    BlackSpace,
    #[strum(serialize = "Little House (Outer Wall)")]
    LittleHouse,
    #[strum(serialize = "Balcony - escaping")]
    BalconyEscaping,
    #[strum(serialize = "Ending")]
    Ending,
    #[strum(serialize = "Attro")]
    Attro,
    #[strum(serialize = "Waterway Cabin")]
    WaterwayCabin,
    #[strum(serialize = "Credits - Labyrinth")]
    CreditsLabyrinth,
    #[strum(serialize = "Credits - Jenka's House")]
    CreditsJenkasHouse,
    #[strum(serialize = "Credits - Power Room")]
    CreditsPowerRoom,
    #[strum(serialize = "Credits - Graveyard")]
    CreditsGraveyard,
    #[strum(serialize = "Credits - Sky")]
    CreditsSky,
    #[strum(serialize = "Prefab House (Balcony) - entrance to hell")]
    PrefabHouseWithEntranceToHell,
    #[strum(serialize = "Sacred Ground B1")]
    SacredGroundB1,
    #[strum(serialize = "Sacred Ground B2")]
    SacredGroundB2,
    #[strum(serialize = "Sacred Ground B3")]
    SacredGroundB3,
    #[strum(serialize = "Storage (Graveyard)")]
    Storage,
    #[strum(serialize = "Passage? - normal")]
    Passage,
    #[strum(serialize = "Passage? - from Sacred Ground B3")]
    PassageFromSacredGroundB3,
    #[strum(serialize = "Statue Chamber (Plantation/Sacred Grounds)")]
    StatueChamber,
    #[strum(serialize = "Seal Chamber (Sacred Grounds) - normal")]
    SealChamber,
    #[strum(serialize = "Corridor (Sacred Grounds)")]
    CorridorSacredGrounds,
    #[strum(serialize = "Credits - Laboratory")]
    CreditsLaboratory,
    #[strum(serialize = "Hermit Gunsmith")]
    HermitGunsmith,
    #[strum(
        serialize = "[empty map, script loaded right before good/best endings]"
    )]
    EmptyMap,
    #[strum(serialize = "Seal Chamber (Sacred Grounds) - after fight")]
    SealChamberAfterFight,
    #[strum(serialize = "Credits - Balcony")]
    CreditsBalcony,
    #[strum(serialize = "Clock Room (Outer Wall)")]
    ClockRoom,
}

impl From<i32> for Map {
    fn from(v: i32) -> Self {
        Map::from_repr(v as u32).unwrap_or(Map::Credits)
    }
}
