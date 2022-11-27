use strum::{Display, EnumIter, FromRepr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Equip(pub u16);

pub trait EquipOpt {
    fn check(&self, equip: Equipment) -> bool;
    fn switch(&mut self, equip: Equipment, state: bool);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumIter, FromRepr)]
#[strum(serialize_all = "title_case")]
pub enum Equipment {
    Boosterv08, //(ignored if v2.0 equipped)
    MapSystem,
    ArmsBarrier,
    Turbocharge,
    CurlysAirTank,
    Boosterv20,
    MaPignon,
    WhimsicalStar,
    NikumaruCounter,
}

impl EquipOpt for Equip {
    fn check(&self, equip: Equipment) -> bool {
        2_u16.pow(equip as u32) & self.0 != 0
    }

    fn switch(&mut self, equip: Equipment, state: bool) {
        if state {
            self.0 |= 2_u16.pow(equip as u32)
        } else {
            self.0 &= !(2_u16.pow(equip as u32))
        }
    }
}
