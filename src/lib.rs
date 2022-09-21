//! check [profile.txt](../profile.txt) for value validation.

#![feature(pointer_byte_offsets)]

mod items;
mod profile;
mod game_profile;

pub use items::*;
pub use game_profile::GameProfile;
pub use profile::Profile;
