# cavestory save lib

modify major values in CaveStory(plus)'s `profile#.dat`

## Example

```rust
use std::fs;
use cavestory_save_lib::GameProfile;
use cavestory_save_lib::Profile;
use cavestory_save_lib::{Weapon, WeaponType};

let mut profile = GameProfile::from(Profile::from(fs::read("profile.dat")?));
profile.max_health = 11451;
profile.weapon[0] = Weapon {
    classification: WeaponType::Spur,
    level: 0, // Spur does not need level, it's
    exp: 0,
};

```

## TODO

- [ ] current song.
- [ ] current map
- [ ] flags
