use std::mem::zeroed;

use crate::profile::Profile;
use crate::Inventory;
use crate::Weapon;

#[derive(Debug, Clone)]
pub struct GameProfile {
    pub health: i16,
    pub max_health: i16,
    pub weapon: [Weapon; 8],
    pub inventory: [Inventory; 32],
    profile: Profile,
}

impl From<Profile> for GameProfile {
    fn from(data: Profile) -> Self {
        let profile = Profile::from(data);

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
            profile,
        }
    }
}

impl From<GameProfile> for Profile {
    fn from(game_profile: GameProfile) -> Self {
        let mut profile = game_profile.profile;
        profile.set_health(game_profile.health);
        profile.set_max_health(game_profile.max_health);

        for (
            slot,
            &Weapon {
                ammo,
                max_ammo,
                exp,
                level,
                classification,
            },
        ) in game_profile.weapon.iter().enumerate()
        {
            profile.set_weapon_ammo(ammo, slot);
            profile.set_weapon_exp(exp, slot);
            profile.set_weapon_level(level, slot);
            profile.set_weapon_max_ammo(max_ammo, slot);
            profile.set_weapon_type(classification as i32, slot);
        }
        for (slot, &inventory) in game_profile.inventory.iter().enumerate() {
            profile.set_inventory(inventory as i32, slot);
        }

        profile
    }
}
