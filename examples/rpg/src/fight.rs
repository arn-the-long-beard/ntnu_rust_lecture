use crate::character::{Character, HealthStatus};
use crate::dice::SkillDice;

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
}

impl Fight {
    pub fn new(first_fighter: Character, second_fighter: Character) -> Self {
        Fight {
            winner_name: None,
            round: 0,
            opponents: (first_fighter, second_fighter),
        }
    }

    pub fn start(&mut self) {
        while self.winner_name.is_none() {
            self.round += 1;

            println!("----------------- round : {} ----------------", self.round);
            // Initiative dice roll.
            let (name1, name2) = self.roll_initiative_dice();

            let (fighter1, fighter2) = if self.opponents.1.name() == name1 {
                (&mut self.opponents.1, &mut self.opponents.0)
            } else {
                (&mut self.opponents.0, &mut self.opponents.1)
            };
            println!("{} will attack during this round", name1);

            println!("{} will defend ", name2);

            // Attack & Dodge dice roll.
            let attack = fighter1.roll_dice(SkillDice::Attack);

            let dodge = fighter2.roll_dice(SkillDice::Dodge);

            if dodge > attack {
                println!(
                    "{} dodged {} attacks successfully",
                    fighter2.name(),
                    fighter1.name()
                );
            } else {
                let damage = fighter1.deal_damages();
                // Block dice roll.
                let def = fighter2
                    .can_block()
                    .map(|_| fighter2.roll_dice(SkillDice::Blocking));

                if def.is_some() {
                    println!("{} tries to block {}", fighter2.name(), fighter1.name());
                }
                fighter2.get_attacked_by(damage, attack, def);
            }

            if fighter2.get_health_status() == HealthStatus::Dead {
                self.winner_name = Some(fighter1.name().to_string());

                println!("{} is dead", fighter2.name());
                println!(
                    "{} won the fight and has {} left",
                    fighter1.name(),
                    fighter1.health()
                );

                println!("Fight finished after {} rounds", self.round);
            }
        }
    }

    // Get the names of attacker and defender
    pub fn roll_initiative_dice(&self) -> (String, String) {
        if self.opponents.1.roll_dice(SkillDice::Initiative)
            > self.opponents.0.roll_dice(SkillDice::Initiative)
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
