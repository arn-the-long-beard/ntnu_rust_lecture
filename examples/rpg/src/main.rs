mod character;
mod dice;
mod fight;
mod item;
mod stuff;
use crate::character::Character;
use crate::fight::Fight;
use item::*;

#[allow(unused)]
fn main() {
    println!("Hello and Fight");

    let iron_plate = BodyArmor::new("Iron Plate", 32.0);
    let steel_plate = BodyArmor::new("Steel Plate", 54.0);

    let iron_long_sword = RegularWeapon::new("Iron Long Sword", 55.0, HandheldType::SingleHand);
    let steel_battle_axe = RegularWeapon::new("Steal battle Axe", 85.0, HandheldType::TwoHands);

    let iron_shield = Shield::new("Iron Shield", 25.0, 5.0);
    let steel_shield = Shield::new("steal Shield", 35.0, 7.0);
    let daedric_armor = BodyArmor::new("Daedric Shield", 45.0);
    let steel_armor = BodyArmor::new("Steel armor", 35.0);
    let iron_armor = BodyArmor::new("Iron Armor", 30.0);
    let grand_ma_skyrim = Character::new("Skyrim Grandma", 1500.00)
        .grab_weapon(steel_battle_axe)
        .grab_armor(daedric_armor);

    let white_run_guard = Character::new("Olaf the dummy guard", 1500.00)
        .grab_weapon(iron_long_sword)
        .grab_weapon(iron_shield)
        .grab_armor(steel_armor);

    Fight::new(grand_ma_skyrim, white_run_guard).start();
}

//https://en.uesp.net/wiki/Skyrim:Block#Defensive_Blocking
