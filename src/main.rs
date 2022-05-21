use std::process::exit;

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
            vec![
                "Current Health",
                "Max Health",
                "Weapon Type",
                "Weapon Level",
                "Weapon Exp Level",
                "Weapon Ammo",
                "Weapon Max Ammo",
                "Inventory",
                "* Save",
                "* Exit",
            ],
        )
        .prompt()?;
        match option {
            "Current Health" => profile.current_health(type_int_value("new value: ")?),
            "Max Health" => profile.max_health(type_int_value("new value: ")?),
            "Weapon Type" => {
                let slot = type_int_value("slot to edit:")? as isize;
                profile.weapon_type(select_id(&WEAPON)?, slot)
            }
            "Weapon Level" => {
                let slot = type_int_value("slot to edit:")? as isize;
                let level = type_int_value("new level: ")?;
                profile.weapon_level(level, slot);
            }
            "* Save" => profile.write_to(path)?,
            "* Exit" => exit(0),
            _ => println!("Unknown choice!"),
        }
    }
}

fn type_int_value(tip: &str) -> Result<i32, Box<dyn std::error::Error>> {
    loop {
        if let Ok(n) = Text::new(tip).prompt()?.trim().parse::<i32>() {
            return Ok(n);
        }
    }
}

fn select_id(list: &Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let opt = Select::new("select the new value: ", list.clone()).prompt()?;
    Ok(list
        .iter()
        .enumerate()
        .find_map(|(i, &op)| if op == opt { Some(i) } else { None })
        .ok_or_else(|| "cannot find this option!")? as i32)
}
