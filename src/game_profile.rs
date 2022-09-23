use std::mem::zeroed;

use crate::profile::Profile;
use crate::items::Inventory;
use crate::items::Weapon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameProfile {
    /// your current health in game.
    pub health: i16,
    
    /// set to non-positive for god mode.
    pub max_health: i16,

    /// you should not set 8th weapon, or it may turn into issue.
    pub weapon: [Weapon; 8],

    /// you should not set 32th inventory, or it may turn into issue.
    pub inventory: [Inventory; 32],
}

impl GameProfile {
    pub fn dump(profile: &Profile) -> Self {
        let health = profile.health();
        let max_health = profile.max_health();
        let mut weapon: [Weapon; 8] = unsafe { zeroed() };
        let mut inventory: [Inventory; 32] = unsafe { zeroed() };

        for i in 0..8 {
            if profile.weapon_type(i) != 0 {
                weapon[i] = Weapon {
                    level: profile.weapon_level(i),
                    ammo: profile.weapon_ammo(i),
                    max_ammo: profile.weapon_max_ammo(i),
                    exp: profile.weapon_exp(i),
                    classification: profile.weapon_type(i).into(),
                };
            }
        }

        for i in 0..32 {
            inventory[i] = profile.inventory(i).into();
        }

        Self {
            health,
            max_health,
            weapon,
            inventory,
        }
    }
}


impl GameProfile {
    pub fn write(&self, profile: &mut Profile) {
        profile.set_health(self.health);
        profile.set_max_health(self.max_health);

        for (
            slot,
            &Weapon {
                ammo,
                max_ammo,
                exp,
                level,
                classification,
            },
        ) in self.weapon.iter().enumerate() {
            profile.set_weapon_ammo(ammo, slot);
            profile.set_weapon_exp(exp, slot);
            profile.set_weapon_level(level, slot);
            profile.set_weapon_max_ammo(max_ammo, slot);
            profile.set_weapon_type(classification as i32, slot);
        }

        for (slot, &inventory) in self.inventory.iter().enumerate() {
            profile.set_inventory(inventory as i32, slot);
        }
    }
}
