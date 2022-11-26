use strum::Display;
use strum::EnumIter;
use strum::FromRepr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[repr(u32)]
pub enum Map {
    #[strum(serialize = "Credits")]
    Credits,
    #[strum(serialize = "Arthur's House")]
    ArthursHouse,
    #[strum(serialize = "Egg Corridor")]
    EggCorridor,
    #[strum(serialize = "Egg No. 00")]
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
    #[strum(serialize = "Labyrinth I")]
    LabyrinthIErticalStartingRoom,
    #[strum(serialize = "Sand Zone")]
    SandZone,
    #[strum(serialize = "Mimiga Village")]
    MimigaVillage,
    #[strum(serialize = "First Cave")]
    #[default]
    FirstCave,
    #[strum(serialize = "Start Point")]
    StartPoint,
    #[strum(serialize = "Shack")]
    ShackMimigaVillage,
    #[strum(serialize = "Reservoir")]
    Reservoir,
    #[strum(serialize = "Graveyard")]
    Graveyard,
    #[strum(serialize = "Yamashita Farm")]
    YamashitaFarm,
    #[strum(serialize = "Shelter")]
    Shelter,
    #[strum(serialize = "Assembly Hall")]
    AssemblyHall,
    #[strum(serialize = "Save Point (Mimiga Village)")]
    SavePointAtMimigaVillage,
    #[strum(serialize = "Side Room")]
    SideRoom,
    #[strum(serialize = "Cthulhu's Abode")]
    CthulhusAbode,
    #[strum(serialize = "Egg No. 01")]
    EggNo01,
    #[strum(serialize = "Arthur's House - sue using computer")]
    ArthursHouseWithSueUsingComputer,
    #[strum(serialize = "Power Room")]
    PowerRoom,
    #[strum(serialize = "Save Point (Grasstown)")]
    SavePointAtGrasstown,
    #[strum(serialize = "Execution Chamber")]
    ExecutionChamber,
    #[strum(serialize = "Gum")]
    Gum,
    #[strum(serialize = "Sand Zone Residence")]
    SandZoneResidence,
    #[strum(serialize = "Grasstown Hut")]
    GrasstownHut,
    #[strum(serialize = "Main Artery")]
    MainArtery,
    #[strum(serialize = "Small Room")]
    SmallRoom,
    #[strum(serialize = "Jenka's House - before Balrog attack")]
    JenkasHouse,
    #[strum(serialize = "Deserted House")]
    DesertedHouse,
    #[strum(serialize = "Sand Zone Storehouse")]
    SandZoneStorehouse,
    #[strum(serialize = "Jenka's House - after Balrog attack")]
    JenkasHouseAfterBalrogAttack,
    #[strum(serialize = "Sand Zone - after boss fight")]
    SandZoneAfterBossFight,
    #[strum(serialize = "Labyrinth H")]
    LabyrinthH,
    #[strum(serialize = "Labyrinth W")]
    LabyrinthWMainAreaWithShopCamp,
    #[strum(serialize = "Camp")]
    Camp,
    #[strum(serialize = "Clinic Ruins")]
    ClinicRuins,
    #[strum(serialize = "Labyrinth Shop")]
    LabyrinthShop,
    #[strum(serialize = "Labyrinth B")]
    LabyrinthBroomwithbooster,
    #[strum(serialize = "Boulder Chamber")]
    BoulderChamber,
    #[strum(serialize = "Labyrinth M")]
    LabyrinthMLastAreaGaudiEggs,
    #[strum(serialize = "Dark Place")]
    DarkPlace,
    #[strum(serialize = "Core")]
    Core,
    #[strum(serialize = "Waterway")]
    Waterway,
    #[strum(serialize = "Egg Corridor?")]
    QuestionableEggCorridor,
    #[strum(serialize = "Cthulhu's Abode?")]
    QuestionableCthulhusAbode,
    #[strum(serialize = "Egg Observation Room?")]
    QuestionableEggObservationRoom,
    #[strum(serialize = "Egg No. 00?")]
    EggNo00WhenHatched,
    #[strum(serialize = "Outer Wall")]
    OuterWall,
    #[strum(serialize = "Side Room?")]
    QuestionableSideRoom,
    #[strum(serialize = "Storehouse")]
    Storehouse,
    #[strum(serialize = "Plantation")]
    Plantation,
    #[strum(serialize = "Jail No. 1")]
    JailNo1,
    #[strum(serialize = "Hideout")]
    Hideout,
    #[strum(serialize = "Rest Area")]
    RestArea,
    #[strum(serialize = "Teleporter")]
    Teleporter,
    #[strum(serialize = "Jail No. 2")]
    JailNo2,
    #[strum(serialize = "Balcony")]
    Balcony,
    #[strum(serialize = "Last Cave")]
    LastCave,
    #[strum(serialize = "Throne Room")]
    ThroneRoom,
    #[strum(serialize = "The King's Table")]
    TheKingsTable,
    #[strum(serialize = "Prefab House")]
    PrefabHouse,
    #[strum(serialize = "Last Cave Hidden")]
    LastCaveHidden,
    #[strum(serialize = "Black Space")]
    BlackSpace,
    #[strum(serialize = "Little House")]
    LittleHouse,
    #[strum(serialize = "Balcony?")]
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
    #[strum(serialize = "Prefab House - entrance to hell")]
    PrefabHouseWithEntranceToHell,
    #[strum(serialize = "Sacred Ground B1")]
    SacredGroundB1,
    #[strum(serialize = "Sacred Ground B2")]
    SacredGroundB2,
    #[strum(serialize = "Sacred Ground B3")]
    SacredGroundB3,
    #[strum(serialize = "Storage")]
    Storage,
    #[strum(serialize = "Passage?")]
    Passage,
    #[strum(serialize = "Passage? - from Sacred Ground B3")]
    PassageFromSacredGroundB3,
    #[strum(serialize = "Statue Chamber")]
    StatueChamber,
    #[strum(serialize = "Seal Chamber")]
    SealChamber,
    #[strum(serialize = "Corridor")]
    CorridorSacredGrounds,
    #[strum(serialize = "Credits - Laboratory")]
    CreditsLaboratory,
    #[strum(serialize = "Hermit Gunsmith")]
    HermitGunsmith,
    #[strum(serialize = "[script loaded right before good/best endings]")]
    EmptyMap,
    #[strum(serialize = "Seal Chamber - after fight")]
    SealChamberAfterFight,
    #[strum(serialize = "Credits - Balcony")]
    CreditsBalcony,
    #[strum(serialize = "Clock Room")]
    ClockRoom,
}

impl From<i32> for Map {
    fn from(v: i32) -> Self {
        Map::from_repr(v as u32).unwrap_or(Default::default())
    }
}
