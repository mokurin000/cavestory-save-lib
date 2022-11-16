use std::fs;
use std::process::exit;
use cavestory_save::GameProfile;
use cavestory_save::Profile;

use cavestory_save::items::{Weapon, WeaponType, Inventory};


fn main() {
    let mut profile = Profile::from(fs::read("profile.dat").unwrap());

    if !profile.verify() {
        eprintln!("Invalid profile.dat");
        exit(1);
    }

    let mut game_profile = GameProfile::dump(&profile);

    game_profile.max_health = -1; // god mode

    game_profile.weapon[0] = Weapon {
        ammo: 0,
        max_ammo: 0, // infinity ammo

        classification: WeaponType::Spur, // cool weapon

        ..Default::default()
    };

    let _ = WeaponType::Missiles; // user-firendly help on broken values

    game_profile.inventory[0] = Inventory::Boosterv20;

    game_profile.write(&mut profile);

    let bytes: Vec<u8> = profile.into();
    fs::write("profile.dat", &bytes).unwrap();
}