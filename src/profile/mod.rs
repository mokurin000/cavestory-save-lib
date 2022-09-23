mod offset;


/// entire data from `profile.dat`
#[derive(Clone, Debug)]
pub struct Profile(Vec<u8>);

impl Profile {
    fn edit(&mut self, offset: usize, value: i32) {
        let p = self.0.as_mut_ptr() as *mut i32;

        unsafe {
            *p.byte_offset(offset as isize) = if cfg!(target_endian = "little") {
                value
            } else {
                value.swap_bytes()
            }
        };
    }

    fn edit16(&mut self, offset: usize, value: i16) {
        let p = self.0.as_mut_ptr() as *mut i16;

        unsafe {
            *p.byte_offset(offset as isize) = if cfg!(target_endian = "little") {
                value
            } else {
                value.swap_bytes()
            }
        };
    }

    fn read(&self, offset: usize) -> i32 {
        let native: i32 = unsafe { *(self.0.as_ptr() as *const i32).byte_offset(offset as isize) };
        if cfg!(target_endian = "little") {
            native
        } else {
            native.swap_bytes()
        }
    }

    fn read16(&self, offset: usize) -> i16 {
        let native: i16 = unsafe { *(self.0.as_ptr() as *const i16).byte_offset(offset as isize) };
        if cfg!(target_endian = "little") {
            native
        } else {
            native.swap_bytes()
        }
    }

    pub fn set_health(&mut self, health: i16) {
        self.edit16(offset::HEALTH, health);
    }

    pub fn set_max_health(&mut self, max_health: i16) {
        self.edit16(offset::MAX_HEALTH, max_health);
    }

    pub fn set_weapon_type(&mut self, weapon: i32, slot: usize) {
        self.edit(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot, weapon);
    }

    pub fn set_weapon_level(&mut self, level: i32, slot: usize) {
        self.edit(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_exp(&mut self, level: i32, slot: usize) {
        self.edit(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot, level);
    }

    pub fn set_weapon_ammo(&mut self, ammo: i32, slot: usize) {
        self.edit(offset::WEAPON_AMMO + offset::WEAPON_SIZE * slot, ammo);
    }

    pub fn set_weapon_max_ammo(&mut self, max_ammo: i32, slot: usize) {
        self.edit(
            offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot,
            max_ammo,
        );
    }

    pub fn set_inventory(&mut self, inventory: i32, slot: usize) {
        self.edit(offset::INVENTORY_TYPE + 4 * slot, inventory)
    }

    pub fn health(&self) -> i16 {
        self.read16(offset::HEALTH)
    }

    pub fn max_health(&self) -> i16 {
        self.read16(offset::MAX_HEALTH)
    }

    pub fn weapon_type(&self, slot: usize) -> i32 {
        self.read(offset::WEAPON_TYPE + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_level(&self, slot: usize) -> i32 {
        self.read(offset::WEAPON_LEVEL + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_exp(&self, slot: usize) -> i32 {
        self.read(offset::WEAPON_EXP + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_ammo(&self, slot: usize) -> i32 {
        self.read(offset::WEAPON_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn weapon_max_ammo(&self, slot: usize) -> i32 {
        self.read(offset::WEAPON_MAX_AMMO + offset::WEAPON_SIZE * slot)
    }

    pub fn inventory(&self, slot: usize) -> i32 {
        self.read(offset::INVENTORY_TYPE + 4 * slot)
    }
}

impl From<Vec<u8>> for Profile {
    fn from(content: Vec<u8>) -> Self {
        Profile(content)
    }
}

impl From<Profile> for Vec<u8> {
    fn from(profile: Profile) -> Self {
        profile.0
    }
}