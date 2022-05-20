#![allow(dead_code)]

pub mod offset {
    pub static SAVE_LOCATION: isize = 0x08;
}

#[repr(C)]
pub enum Weapon {
    None,
}
