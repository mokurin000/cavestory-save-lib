use cavestory_save::GameProfile;
use cavestory_save::Profile;
use std::fs;

use cavestory_save::items::{Inventory, Weapon, WeaponType};

fn main() {
    match Profile::try_from(fs::read("profile.dat").unwrap()) {
        Ok(mut profile) => {
            let mut game_profile = GameProfile::dump(&profile);

            game_profile.max_health = -1; // god mode

            game_profile.weapon[0] = Weapon {
                ammo: 0,
                max_ammo: 0, // infinity ammo

                classification: WeaponType::Spur, // cool weapon

                ..Default::default()
            };

            game_profile.inventory[0] = Inventory::Boosterv20;

            game_profile.write(&mut profile);

            let bytes: Vec<u8> = profile.into();
            fs::write("profile.dat", &bytes).unwrap();
        }
        Err(e) => eprintln!("{e}"),
    }
}
