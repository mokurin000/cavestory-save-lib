//! check [profile.txt](../profile.txt) for value validation.

#![feature(pointer_byte_offsets)]

pub mod items;
mod profile;
mod game_profile;

pub use game_profile::GameProfile;
pub use profile::Profile;
