use inquire::Text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_in = Text::new("Path of the profile#.dat");
    path_in.default = Some("profile.dat");

    cavestory_save_editor::Profile::new(&path_in.prompt()?)?;

    Ok(())
}
