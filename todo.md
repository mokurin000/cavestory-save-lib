# Todo

currently we support:

- [x] health
  - [x] max health
  - [x] current health
- [x] weapon
  - [x] type
  - [x] level
  - [x] exp
  - [x] max ammo
  - [x] current ammo
- [ ] inventory

and for now this tool need to be run natively,
we expect that it could support cross-platform.

from strategy wiki, profile.dat use little-endian,
idk what is the behavior on big-endian machine, since I haven't found any
endian transmute function in NX Engine.
