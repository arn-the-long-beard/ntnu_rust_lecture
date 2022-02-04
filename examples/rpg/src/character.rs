use crate::dice;
use crate::item::RegularWeapon;
use crate::item::Weapon;
use crate::item::*;
use crate::stuff::{Stuff, StuffConfig};
use caith::Roller;

pub struct Character {
    name: String,
    health: f32,
    max_health: f32,
    stuff: Stuff,
}

impl Character {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn health(&self) -> f32 {
        self.health
    }
    pub fn set_health(&mut self, health: f32) {
        self.health = health;
    }
}

impl Character {
    pub fn new(name: &str, health: f32) -> Self {
        Character {
            name: name.to_string(),
            health,
            max_health: health,
            stuff: Default::default(),
        }
    }

    pub fn grab_weapon<W: Weapon + 'static>(mut self, new_weapon: W) -> Self {
        self.stuff = self.stuff.equip_weapon(new_weapon);
        self
    }

    pub fn grab_armor<A: Armor + 'static>(mut self, armor: A) -> Self {
        self.stuff = self.stuff.equip_armor(armor);
        self
    }

    /// Block damage if the set of weapons allow it.
    /// We could have specials skills there to unlock to apply as modified on the amount of blocked damages.
    fn try_to_block(&self) -> Option<BlockedDamages> {
        match self.stuff.get_weapon_settings() {
            StuffConfig::DualWeapons => {
                None // Could have skill to unlock blocking with dual weapon
            }
            StuffConfig::ShieldAndWeapon => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::TwoHandsWeapon => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::OnlyShied => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::OneSingleHandWeapon => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::OneWeaponAsSecondary => None,
        }
    }

    pub fn get_attacked_by(&mut self, damages: RawDamages, attack_dice: u8) {
        // We could have armor skills to add to the calculation
        let mut receive_damage = damages - self.get_armor();

        if let Some(blocking_damage) = self.try_to_block() {
            println!("{} will try to block the attack", self.name);

            let blocking_dice_result = dice::SkillDice::Initiative.dices_roll_result();

            if blocking_dice_result < attack_dice {
                eprintln!("{} failed blocking the attack ", self.name());
            } else {
                // We could have armor blocking to add to the calculation

                receive_damage -= blocking_damage;
                println!(
                    "{} blocked {} with its weapon",
                    self.name(),
                    blocking_damage
                )
            }
        }
        println!("{} received {}", self.name, receive_damage);
        self.health -= receive_damage;

        if self.health < 0.0 {
            self.health = 0.0
        }
    }
    fn get_armor(&self) -> BlockedDamages {
        self.stuff.get_armor_rating()
    }

    /// Get a status to describe the health of the character.
    pub fn get_status(self) -> HealthStatus {
        let percentage: u8 = ((self.max_health - self.health) / self.max_health) as u8 * 100;

        match percentage {
            0 => HealthStatus::Dead,
            1..=10 => HealthStatus::AlmostDead,
            11..=30 => HealthStatus::SeriouslyHurt,
            31..=50 => HealthStatus::VeryHurt,

            51..=75 => HealthStatus::VeryHurt,
            76..=99 => HealthStatus::SlightlyHurt,
            100 => HealthStatus::Healthy,
            _ => {
                //Rust require us to
                println!(
                    "{} % of maximum health, Did you get some magic ?",
                    percentage
                );
                HealthStatus::Healthy
            }
        }
    }
    pub fn deal_damages(&self) -> RawDamages {
        self.stuff.calculate_damages()
    }
}

pub enum HealthStatus {
    Dead,
    AlmostDead,
    SeriouslyHurt,
    VeryHurt,
    LightlyHurt,
    SlightlyHurt,
    Healthy,
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_player() -> Character {
        Character::new("test", 1000.0)
    }
}
