use std::fs;

use cavestory_save_lib::GameProfile;
use cavestory_save_lib::Inventory;
use cavestory_save_lib::Profile;
use cavestory_save_lib::Weapon;
use cavestory_save_lib::WeaponType;

fn main() {
    let profile = Profile::from(fs::read("profile.dat").unwrap());
    let mut profile = GameProfile::from(profile);

    profile.max_health = -1; // god mode

    profile.weapon[0] = Weapon {
        ammo: 0,
        max_ammo: 0, // infinity ammo

        classification: WeaponType::Spur, // cool weapon

        ..Default::default()
    };

    let _ = WeaponType::Missiles; // user-firendly help on broken values

    profile.inventory[0] = Inventory::Boosterv20;

    let v: Vec<u8> = Profile::from(profile).into();
    fs::write("profile.dat", v).unwrap();
}