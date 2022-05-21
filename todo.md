# Todo

currently we support:

- [x] health
  - [x] max health
  - [x] current health
- [ ] weapon
  - [x] type
  - [x] level
  - [ ] exp
  - [ ] max ammo
  - [ ] current ammo
- [ ] inventory

and for now this tool need to be run natively,
we expect that it could support cross-platform.

from strategy wiki, profile.dat use little-endian,
idk what is the behavior on big-endian machine, since I haven't found any
endian transmute function in NX Engine.
