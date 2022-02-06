use crate::character::{Character, HealthStatus};
use crate::dice::SkillDice;
use std::collections::HashMap;

pub struct Fight {
    winner_name: Option<String>,
    round: u16,
    opponents: (Character, Character),
}

#[allow(unused)]
impl Fight {
    pub fn winner_name(&self) -> &Option<String> {
        &self.winner_name
    }
    pub fn round(&self) -> u16 {
        self.round
    }
    pub fn opponents(&self) -> &(Character, Character) {
        &self.opponents
    }

    pub fn new(first_fighter: Character, second_fighter: Character) -> Self {
        Fight {
            winner_name: None,
            round: 0,
            opponents: (first_fighter, second_fighter),
        }
    }

    pub fn start(&mut self) {
        let mut attack_counting: HashMap<String, u32> = HashMap::new();
        attack_counting.insert(self.opponents.1.name().to_string(), 0);
        attack_counting.insert(self.opponents.0.name().to_string(), 0);
        while self.winner_name.is_none() {
            self.round += 1;

            println!("----------------- round : {} ----------------", self.round);
            // Initiative dice roll.
            let (attacker_name, defender_name) = self.roll_initiative_dice();

            let (attacker, defender) = if self.opponents.1.name() == attacker_name {
                (&mut self.opponents.1, &mut self.opponents.0)
            } else {
                (&mut self.opponents.0, &mut self.opponents.1)
            };

            let number_attacks = &attack_counting
                .get(&attacker_name)
                .expect("Should have gotten number of attack for given name");

            let new_nb = *number_attacks + 1;
            attack_counting.insert(attacker_name.clone(), new_nb);

            println!("{} will attack during this round", attacker_name.clone());

            println!("{} will defend ", defender_name);

            // Attack & Dodge dice roll.
            let attack = attacker.roll_dice(SkillDice::Attack);

            let dodge = defender.roll_dice(SkillDice::Dodge);

            if dodge > attack {
                println!(
                    "{} dodged {} attacks successfully",
                    defender.name(),
                    attacker.name()
                );
            } else {
                let damage = attacker.deal_damages();
                // Block dice roll.
                let def = defender
                    .can_block()
                    .map(|_| defender.roll_dice(SkillDice::Blocking));

                if def.is_some() {
                    println!("{} tries to block {}", defender.name(), attacker.name());
                }
                defender.get_attacked_by(damage, attack, def);
            }

            if defender.get_health_status() == HealthStatus::Dead {
                self.winner_name = Some(attacker.name().to_string());

                println!("{} is dead  :((((((((", defender.name());
                println!(
                    "{} won the fight and has {} hp left <3 !!!!!!!!!!!!!!!",
                    attacker.name(),
                    attacker.health()
                );

                println!(" -------------- Game statistics  -------------- ");

                println!("Fight finished after {} rounds", self.round);

                let defender_nb_attacks = attack_counting
                    .get(&defender_name)
                    .expect("Should have gotten defender attack number");
                let attacker_nb_attacks = attack_counting
                    .get(&attacker_name)
                    .expect("Should have gotten defender attack number");
                println!(
                    " {} has attacked {} times ",
                    defender_name, defender_nb_attacks
                );
                println!(
                    " {} has attacked {} times ",
                    attacker_name, attacker_nb_attacks
                );
                println!()
            }
        }
    }

    // Get the names of attacker and defender
    // So ugly
    pub fn roll_initiative_dice(&self) -> (String, String) {
        if self.opponents.1.roll_dice(SkillDice::Initiative)
            >= self.opponents.0.roll_dice(SkillDice::Initiative)
        {
            (
                self.opponents.1.name().to_string(),
                self.opponents.0.name().to_string(),
            )
        } else {
            (
                self.opponents.0.name().to_string(),
                self.opponents.1.name().to_string(),
            )
        }
    }
}
