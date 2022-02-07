use colored::*;
use rpg_engine::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(feature = "song")]
fn add_song() {
    println!(
        "Dovahkiin, Dovahkiin, naal ok zin los vahriin,
    Wah dein vokul mahfaeraak ahst vaal!
    Ahr fin norok paal graan fod nust hon zindro zaan,
    Dovahkiin fah hin kogaan mu draal!"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "
    Huzrah nu, kul do od, wah aan bok lingrah vod,
    Ahrk fin tey, boziik fun, do fin gein!
    Wo lost fron wah ney dov, ahrk fin reyliik do jul,
    Voth aan suleyk wah ronit faal krein!"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "    Ahrk fin zul, rok drey kod, nau tol morokei frod,
    Rul lot Taazokaan motaad voth kein!
    Sahrot Thu'um, med aan tuz, bey zeim hokoron pah,
    Ol fin Dovahkiin Komeyt ok rein!
"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "  Dovahkiin, Dovahkiin, naal ok zin los vahriin,
    Wah dein vokul mahfaeraak ahst vaal!
    Ahr fin norok paal graan fod nust hon zindro zaan,
    Dovahkiin fah hin kogaan mu draal!
"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "  Ark fin Kel lost prodah, do ved viing ko fin krah,
    Tol fod zeymah win kein meyz fundein!
    Alduin, feyn do jun, kruziik vokun staadnau,
    Voth aan bahlok wah diivon fin lein!"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "   Nuz aan sul, fent alok, fod fin vul dovah nok,
    Fen kos nahlot mahfaeraak ahrk ruz!
    Paaz Keizaal fen kos stin nol bein Alduin jot,
    Dovahkiin kos fin saviik do muz!
"
    );
    thread::sleep(Duration::new(2, 0));
    println!(
        "    Dovahkiin, Dovahkiin, naal ok zin los vahriin,
    Wah dein vokul mahfaeraak ahst vaal!
    Ahr fin norok paal graan fod nust hon zindro zaan,
    Dovahkiin fah hin kogaan mu draal!"
    );
}

fn main() {
    let iron_plate = BodyArmor::new("Iron Plate", 32.0);
    let steel_plate = BodyArmor::new("Steel Plate", 54.0);
    let daedric_armor = BodyArmor::new("Daedric Armor", 25.0);
    let daedric_armor_2 = BodyArmor::new("Daedric Armor 2", 25.0);

    // Lets put some shields
    let steel_shield = Shield::new("steal Shield", 55.0, 20.0);
    let iron_shield = Shield::new("Iron Shield", 25.0, 15.0);

    // Lets put some weapons.
    let steel_long_sword = RegularWeapon::new("Steel Long Sword", 40.0, HandheldType::SingleHand);
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

    let dovahkiin = Character::new("Dovahkiin", 1500.0)
        .grab_weapon(steel_long_sword)
        .grab_weapon(iron_shield);

    let (tx_1, rx_1) = mpsc::channel();

    // This is OS native Thread
    let _ = thread::spawn(move || {
        let winner = Fight::new(white_run_guard, grand_ma_skyrim).resolve();
        tx_1.send(winner)
            .expect("Should have passed the resolved winner");
    });

    let (tx_2, rx_2) = mpsc::channel();

    // This is OS native Thread
    let _ = thread::spawn(move || {
        let winner = Fight::new(lydia, dovahkiin).resolve();
        tx_2.send(winner)
            .expect("Should have passed the resolved winner");
    });

    let second_fight_winner = rx_2.recv().expect("Should have receive the winner");
    let first_fight_winner = rx_1.recv().expect("Should have receive the winner");

    println!("----------------- Final Fight ----------------- ");
    println!(
        "{} and {} will fight until only one survive",
        first_fight_winner.name().bold(),
        second_fight_winner.name().bold(),
    );

    println!(
        "{} has {} HP",
        first_fight_winner.name().bold(),
        first_fight_winner.health().to_string().green().bold(),
    );
    println!(
        "{} has {} HP",
        second_fight_winner.name().bold(),
        second_fight_winner.health().to_string().green().bold(),
    );
    println!();

    // Use the stuff bellow to make it work sequential
    //first_fight.join().unwrap();
    //second_fight.join().unwrap();
    let final_winner = Fight::new(first_fight_winner, second_fight_winner).resolve();

    println!(
        "The best fighter is : {}",
        final_winner.name().yellow().bold()
    );

    #[cfg(feature = "song")]
    add_song();
}
