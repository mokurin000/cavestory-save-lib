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
            vec!["Current Health", "Max Health", "* Save", "* Exit"],
        )
        .prompt()?;
        match option {
            "Current Health" => profile.current_health(type_int_value()?),
            "Max Health" => profile.max_health(type_int_value()?),
            "* Save" => profile.write_to(path)?,
            "* Exit" => exit(0),
            _ => println!("Unknown choice!"),
        }
    }
}

fn type_int_value() -> Result<i32, Box<dyn std::error::Error>> {
    loop {
        if let Ok(n) = Text::new("type the new value: ")
            .prompt()?
            .trim()
            .parse::<i32>()
        {
            return Ok(n);
        }
    }
}
