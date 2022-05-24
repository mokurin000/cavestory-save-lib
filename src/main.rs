use std::{process::exit, str::FromStr};

use cavestory_save_editor::{Profile, INVENTORY, WEAPON};
use inquire::{Select, Text};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_in = Text::new("Path of the profile#.dat");
    path_in.default = Some("profile.dat");
    let path = &path_in.prompt()?;

    let mut profile = Profile::new(path)?;

    loop {
        let option = Select::new(
            ": ",
            vec!["Health", "Weapon", "Inventory", "* Save", "* Exit"],
        )
        .prompt()?;
        match option {
            "Health" => {
                profile.set_health(type_value("current:", profile.health())?);
                profile.set_max_health(type_value("max:", profile.max_health())?)
            }
            "Weapon" => {
                let slot = type_value("slot to edit:", 0)? as isize;

                profile.set_weapon_type(select_id(&WEAPON, profile.weapon_type(slot))?, slot);

                let level = type_value("level:", profile.weapon_level(slot))?;
                profile.set_weapon_level(level, slot);

                let exp_level = type_value("exp level:", profile.weapon_exp(slot))?;
                profile.set_weapon_exp(exp_level, slot);

                let ammo = type_value("ammo:", profile.weapon_ammo(slot))?;
                profile.set_weapon_ammo(ammo, slot);

                let max_ammo = type_value("max ammo:", profile.weapon_max_ammo(slot))?;
                profile.set_weapon_max_ammo(max_ammo, slot);
            }
            "Inventory" => {
                let slot = type_value("slot to edit:", 0)? as isize;

                let inventory = select_id(&INVENTORY, profile.inventory(slot))?;
                profile.set_inventory(inventory, slot);
            }
            "* Save" => profile.write_to(path)?,
            "* Exit" => exit(0),
            _ => println!("Unimplemented choice!"),
        }
    }
}

fn type_value<T>(tip: &str, default: T) -> Result<T, Box<dyn std::error::Error>>
where
    T: Copy + Display + FromStr,
{
    loop {
        if let Ok(n) = Text::new(tip)
            .with_default(&default.to_string())
            .prompt()?
            .trim()
            .parse::<T>()
        {
            return Ok(n);
        }
    }
}

fn select_id(list: &[&str], default: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let opt = Select::new("select the new value:", list.to_vec()).prompt_skippable()?;
    Ok(opt
        .map(|opt| list.iter().position(|&option| opt == option).unwrap() as i32)
        .unwrap_or(default))
}
