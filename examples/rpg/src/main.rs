mod character;
mod item;
mod stuff;
//use crate::character::Character;
use item::*;

fn main() {
    println!("Hello and Fight");

    let iron_plate = BodyArmor::new("Iron Plate", 32.0);
    let steel_plate = BodyArmor::new("Steel Plate", 54.0);

    let iron_long_sword = RegularWeapon::new("Iron Long Sword", 25.0, HandheldType::SingleHand);
    let steel_battle_aze = RegularWeapon::new("Steal battle Axe", 65.0, HandheldType::TwoHands);

    let iron_shield = Shield::new("Iron Shield", 25.0, 5.0);
    let steel_shield = Shield::new("steal Shield", 35.0, 7.0);

    // let grand_ma_skyrim = Character::<RegularWeapon>::new("Skyrim Grandma", 1500.00);
    //
    // let white_run_guard = Character::<RegularWeapon>::new("Olaf the dummy guard", 1500.00);
}

//https://en.uesp.net/wiki/Skyrim:Block#Defensive_Blocking
