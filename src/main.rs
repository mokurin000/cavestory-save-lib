use cavestory_save_editor::{Profile, WEAPONS};
use inquire::Text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_in = Text::new("Path of the profile#.dat");
    path_in.default = Some("profile.dat");

    let mut profile = Profile::new(&path_in.prompt()?)?;

    profile.current_health(50);

    Ok(())
}
