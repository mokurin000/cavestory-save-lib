# cavestory save lib

[![](https://img.shields.io/docsrs/cavestory-save-lib)](https://docs.rs/cavestory-save-lib/latest/cavestory_save/)

modify major values in CaveStory(plus)'s `profile.dat`

## Note

As `impl From ...` in `src/items/*.rs`, if you get invalid value from `profile.dat`, it will set to `None`(Nothing for BGM).

## TODO

- [ ] current map
- [ ] Teleporter
- [ ] equipments - currently not in plan, since inventory provides options for equipments
- [ ] flags ([`bitvec`](https://docs.rs/bitvec/latest/bitvec/)) - currently not in plan, as modify this may break game experience
