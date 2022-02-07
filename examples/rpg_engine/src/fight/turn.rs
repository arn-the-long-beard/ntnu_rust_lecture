use crate::character::{Character, HealthStatus};
use crate::dice::SkillDice;
use crate::item::{BlockedDamages, RawDamages};
use colored::*;

pub struct Turn {
    pub attacker: Character,
    pub defender: Character,
    pub number: u32,
}

impl Turn {
    pub fn new(number: u32, attacker: Character, defender: Character) -> Self {
        let turn = Turn {
            attacker,
            defender,
            number,
        };

        turn.display_results(TurnStep::Initiative);
        turn
    }

    pub fn resolve_attack_defense(&self) -> Option<(u8, RawDamages)> {
        let attack = self.attacker.roll_dice(SkillDice::Attack);
        let dodge = self.defender.roll_dice(SkillDice::Dodge);

        if dodge > attack {
            self.display_results(TurnStep::Dodge);
            None
        } else {
            self.display_results(TurnStep::Attack);
            Some((attack, self.attacker.deal_damages()))
        }
    }

    /// Roll dice and display message for success or fail .
    /// And return update damages.
    pub fn resolve_blocking(
        &mut self,
        attack: u8,
        can_block: BlockedDamages,
        damage: RawDamages,
    ) -> Option<RawDamages> {
        self.display_results(TurnStep::TryBlocking);
        let blocking = self.defender.roll_dice(SkillDice::Blocking);
        if blocking >= attack {
            self.display_results(TurnStep::ResultBlocking {
                succeed: true,
                blocked_damage: can_block,
            });

            let mut hit_damage = damage - can_block;

            // Did not fix max cap for now.
            if hit_damage < 0.0 {
                hit_damage = 0.0;
            }
            Some(hit_damage)
        } else {
            self.display_results(TurnStep::ResultBlocking {
                succeed: false,
                blocked_damage: damage,
            });
            None
        }
    }

    pub fn resolve_damages(&mut self, hit_damage: RawDamages) {
        let result = self.defender.gets_hit(hit_damage);
        self.display_results(TurnStep::DamagesTaken { damages: result })
    }

    pub fn resolve_winner_and_loser(&self) -> Option<(Character, Character)> {
        if self.defender.get_health_status() == HealthStatus::Dead {
            self.display_results(TurnStep::EndFight);
            Some((self.attacker.clone(), self.defender.clone()))
        } else {
            None
        }
    }

    /// Should use string template make unit test.
    /// Displays specific message for a particular step during the turn.
    fn display_results(&self, turn_step: TurnStep) {
        let attacker_name = &self.attacker.name();
        let defender_name = &self.defender.name();
        match turn_step {
            TurnStep::Initiative => {
                println!(
                    "{} will attack and {} will defend",
                    attacker_name.bold(),
                    defender_name.bold()
                );
            }
            TurnStep::Attack => {
                println!(
                    "{} {} {} {} attack",
                    defender_name.bold(),
                    "failed".red(),
                    "to dodge".underline(),
                    attacker_name.bold(),
                );
            }

            TurnStep::Dodge => {
                println!(
                    "{} dodged {} attack {}",
                    defender_name.bold(),
                    attacker_name.bold(),
                    "successfully".green()
                );
            }
            TurnStep::TryBlocking => {
                println!(
                    "{} tries to block {}",
                    defender_name.bold(),
                    attacker_name.bold()
                );
            }

            TurnStep::ResultBlocking {
                blocked_damage,
                succeed,
            } => {
                if succeed {
                    println!(
                        "{} {} {} {} damages from {}",
                        defender_name.bold(),
                        "succeed".green(),
                        "to block".underline(),
                        blocked_damage.to_string().red(),
                        attacker_name.bold()
                    );
                } else {
                    println!(
                        "{} {} to block {} from {}",
                        defender_name.bold(),
                        "failed".red(),
                        blocked_damage.to_string().red(),
                        attacker_name.bold(),
                    );
                }
            }
            TurnStep::DamagesTaken { damages } => {
                println!(
                    "{} deals {} {} to {}",
                    attacker_name.bold(),
                    damages.to_string().red(),
                    "damages".red(),
                    defender_name.bold()
                );
            }
            TurnStep::EndFight => {
                println!(
                    "{} is {}  :((((((((",
                    &self.defender.name().bold(),
                    "dead".red().bold()
                );
                println!(
                    "{} {} the fight and has {} hp left <3 !!!!!!!!!!!!!!!",
                    &self.attacker.name().bold(),
                    "won".yellow().bold(),
                    &self.attacker.health().to_string().green().bold()
                );
            }
        }
    }
}

pub enum TurnStep {
    Initiative,
    Attack,
    Dodge,
    TryBlocking,
    ResultBlocking {
        blocked_damage: BlockedDamages,
        succeed: bool,
    },

    DamagesTaken {
        damages: BlockedDamages,
    },
    EndFight,
}
