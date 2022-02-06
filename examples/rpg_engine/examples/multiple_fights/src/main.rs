use rpg_engine::prelude::*;
use std::sync::mpsc;
use std::thread;

fn main() {
    let iron_plate = BodyArmor::new("Iron Plate", 32.0);
    let steel_plate = BodyArmor::new("Steel Plate", 54.0);
    let daedric_armor = BodyArmor::new("Daedric Armor", 25.0);
    let daedric_armor_2 = BodyArmor::new("Daedric Armor 2", 25.0);

    // Lets put some shields
    let steel_shield = Shield::new("steal Shield", 55.0, 20.0);
    let iron_shield = Shield::new("Iron Shield", 25.0, 15.0);

    // Lets put some weapons.
    let iron_long_sword = RegularWeapon::new("Iron Long Sword", 35.0, HandheldType::SingleHand);
    let steel_battle_axe = RegularWeapon::new("Steel battle Axe", 65.0, HandheldType::TwoHands);
    let daedric_battle_axe = RegularWeapon::new("Daedric battle Axe", 85.0, HandheldType::TwoHands);

    let grand_ma_skyrim = Character::new("Skyrim Grandma", 300.00)
        .grab_weapon(steel_battle_axe)
        .grab_armor(daedric_armor);

    let white_run_guard = Character::new("Olaf the dummy guard", 300.00)
        .grab_weapon(steel_shield) // <- we can do it because of generic + trait objects for weapon
        .grab_weapon(iron_long_sword)
        .grab_armor(daedric_armor_2);

    let lydia = Character::new("Lydia", 300.0)
        .grab_weapon(daedric_battle_axe)
        .grab_armor(iron_plate);

    let braith = Character::new("Braith", 100.0).grab_weapon(RegularWeapon::default());

    let first_fight = thread::spawn(move || {
        Fight::new(white_run_guard, grand_ma_skyrim).start();
    });

    let second_fight = thread::spawn(move || {
        Fight::new(lydia, braith).start();
    });

    first_fight.join().unwrap();
    second_fight.join().unwrap();
}
