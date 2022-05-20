
use std::{collections::BTreeMap, lazy::SyncLazy};


pub static WEAPONS: SyncLazy<BTreeMap<&'static str, i32>> = SyncLazy::new(|| {
    let mut map = BTreeMap::new();
    map.insert("None",0);
    map.insert("Snake",1);
    map.insert("Polar Star",2);
    map.insert("Fireball",3);
    map.insert("Machine Gun",4);
    map.insert("Missile Launcher",5);
    map.insert("Missiles [Bad]",6);
    map.insert("Bubbler",7);
    map.insert("Unknown [Bad]",8);
    map.insert("Blade",9);
    map.insert("Super Missile",10);
    map.insert("Super Missiles [Bad]",11);
    map.insert("Nemesis",12);
    map.insert("Spur",13);
    map.insert("\"Hajime\" [Bad]",14);
    map
});

pub static MAPS: SyncLazy<BTreeMap<&'static str, i32>> = SyncLazy::new(||{
    let mut map = BTreeMap::new();
    map.insert("None", 0);
    map.insert("Arthur's Key", 1);
    map.insert("Map System", 2);
    map.insert("Santa's Key", 3);
    map.insert("Silver Locket", 4);
    map.insert("Beast Fang", 5);
    map.insert("Life Capsule", 6);
    map.insert("ID Card", 7);
    map.insert("Jellyfish Juice", 8);
    map.insert("Rusted Key", 9);
    map.insert("Gum Key", 10);
    map.insert("Gum Base", 11);
    map.insert("Charcoal", 12);
    map.insert("Bomb", 13);
    map.insert("Dog", 14);
    map.insert("Life Pot", 15);
    map.insert("Cure-All", 16);
    map.insert("Clinic Key", 17);
    map.insert("Booster v0.8", 18);
    map.insert("Arms Barrier", 19);
    map.insert("Turbocharge", 20);
    map.insert("Air Tank", 21);
    map.insert("290 Counter", 22);
    map.insert("Booster v2.0", 23);
    map.insert("Mimiga Mask", 24);
    map.insert("Teleporter Room Key", 25);
    map.insert("Sue's Letter", 26);
    map.insert("Controller", 27);
    map.insert("Broken Sprinkler", 28);
    map.insert("Sprinkler", 29);
    map.insert("Tow Rope", 30);
    map.insert("Medal of the Red Ogre", 31);
    map.insert("Mister Little", 32);
    map.insert("Mushroom Badge", 33);
    map.insert("Ma Pignon", 34);
    map.insert("Curly's Panties", 35);
    map.insert("Alien Medal", 36);
    map.insert("Chako's Rouge", 37);
    map.insert("Whimsical Star", 38);
    map.insert("Iron Bond", 39);
    map
});