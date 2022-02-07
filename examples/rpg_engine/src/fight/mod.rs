use crate::character::{Character, HealthStatus};
use crate::dice::SkillDice;
use crate::item::{BlockedDamages, RawDamages};
use colored::*;

pub mod turn;
use crate::fight::turn::{Turn, TurnStep};
use std::collections::HashMap;

pub struct Fight {
    winner_name: Option<String>,
    round: u32,
    attack_counting: HashMap<String, u32>,
    opponents: (Character, Character),
    winner: Option<Character>,
    loser: Option<Character>,
}

#[allow(unused)]
impl Fight {
    pub fn winner_name(&self) -> &Option<String> {
        &self.winner_name
    }
    pub fn round(&self) -> u32 {
        self.round
    }
    pub fn opponents(&self) -> &(Character, Character) {
        &self.opponents
    }

    pub fn new(first_fighter: Character, second_fighter: Character) -> Self {
        let mut attack_counting = HashMap::new();
        attack_counting.insert(first_fighter.name().to_string(), 0);
        attack_counting.insert(second_fighter.name().to_string(), 0);
        Fight {
            winner_name: None,
            round: 0,
            attack_counting,
            opponents: (first_fighter, second_fighter),
            winner: None,
            loser: None,
        }
    }

    fn update_attacks_counter(&mut self, attack_name: &str) {
        if let Some(number_attacks) = self.attack_counting.get_mut(attack_name) {
            *number_attacks += 1;
        }
    }

    /// SO ugly code, please use `resolve()` instead.
    #[deprecated]
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
                    "{} dodged {}'s attack successfully",
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

    pub fn resolve(mut self) -> Character {
        while self.winner_name.is_none() {
            self.round += 1;

            println!(
                "{}{} starts",
                "turn ".underline(),
                self.round.to_string().underline()
            );
            let (attacker, defender) = self.resolve_initiative();

            let mut turn = Turn::new(self.round, attacker, defender);

            self.update_attacks_counter(turn.attacker.name());
            if let Some((attack, damage)) = turn.resolve_attack_defense() {
                let mut hit_damage = damage;

                if let Some(max_blocking) = turn.defender.can_block() {
                    if let Some(reduced_damage) =
                        turn.resolve_blocking(attack, max_blocking, damage)
                    {
                        hit_damage = reduced_damage;
                    }
                }
                turn.resolve_damages(hit_damage);

                if let Some((winner, loser)) = turn.resolve_winner_and_loser() {
                    self.winner_name = Some(winner.name().to_string());
                    self.winner = Some(winner);
                    self.loser = Some(loser);
                    self.show_statistics()
                }
            }
            // We cloned them in the first place so we need to have then.
            self.update_opponents(turn.attacker, turn.defender);
        }

        self.winner.unwrap()
    }

    /// Update the stored value with new ones.
    fn update_opponents(&mut self, fighter_1: Character, fighter_2: Character) {
        self.opponents = (fighter_1, fighter_2);
    }

    /// SHow simple statistics.
    fn show_statistics(&self) {
        println!(" -------------- Game statistics  -------------- ");

        println!("Fight finished after {} rounds", self.round);

        match (&self.winner, &self.loser) {
            (Some(winner), Some(loser)) => {
                let loser_name = loser.name();
                let winner_name = winner.name();

                let loser_nb_attacks = self
                    .attack_counting
                    .get(loser_name)
                    .expect("Should have gotten winner attack number");
                let winner_nb_attacks = self
                    .attack_counting
                    .get(winner_name)
                    .expect("Should have gotten loser attack number");
                println!(
                    " {} has attacked {} times ",
                    loser_name.bold(),
                    loser_nb_attacks.to_string().underline()
                );
                println!(
                    " {} has attacked {} times ",
                    winner_name.bold(),
                    winner_nb_attacks.to_string().underline()
                );
                println!()
            }
            _ => {
                panic!("Cannot display statistics because winner and loser are not resolved")
            }
        }
    }

    /// Consume the tuple to return a new one with attacker and then defender.
    fn resolve_initiative(&self) -> (Character, Character) {
        let first = self.opponents.0.roll_dice(SkillDice::Initiative);
        let second = self.opponents.1.roll_dice(SkillDice::Initiative);

        if first >= second {
            (self.opponents.1.clone(), self.opponents.0.clone())
        } else {
            (self.opponents.0.clone(), self.opponents.1.clone())
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::prelude::*;

    fn get_test_player(name: &str) -> Character {
        Character::new(name, 1000.0).grab_weapon(RegularWeapon::default())
    }
    fn get_long_iron_sword() -> RegularWeapon {
        RegularWeapon::new("Long Iron Sword", 25.0, HandheldType::SingleHand)
    }
    fn get_long_steel_sword() -> RegularWeapon {
        RegularWeapon::new("Long Steel Sword", 30.0, HandheldType::SingleHand)
    }
    fn get_steel_battle_axe() -> RegularWeapon {
        RegularWeapon::new("Steal battle Axe", 65.0, HandheldType::TwoHands)
    }

    fn get_iron_shield() -> Shield {
        Shield::new("Iron Shield", 25.0, 5.0)
    }

    fn get_steel_shield() -> Shield {
        Shield::new("Steel Shield", 35.0, 7.0)
    }

    fn get_daedric_mail() -> BodyArmor {
        BodyArmor::new("Daedric Shield", 45.0)
    }

    #[test]
    fn start() {
        let winner = Fight::new(get_test_player("player 1"), get_test_player("player 2")).resolve();
        assert_eq!(winner.name(), "player 1");
    }
}
