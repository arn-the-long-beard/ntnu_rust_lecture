use crate::dice::SkillDice;
use crate::item::Weapon;
use crate::item::*;
use crate::stuff::{Stuff, StuffConfig};

pub struct Character {
    name: String,
    health: f32,
    max_health: f32,
    stuff: Stuff,
}

#[allow(unused)]
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

#[allow(unused)]
impl Character {
    pub fn new(name: &str, health: f32) -> Self {
        Character {
            name: name.to_string(),
            health,
            max_health: health,
            stuff: Default::default(),
        }
    }

    /// Roll specific dice
    /// Todo : We could add here skills modified
    pub fn roll_dice(&self, skill: SkillDice) -> u8 {
        skill.dices_roll_result(&self.name)
    }

    pub fn grab_weapon<W: Weapon + 'static>(mut self, new_weapon: W) -> Self {
        self.stuff = self.stuff.equip_weapon(new_weapon);
        self
    }

    pub fn drop_first_weapon(mut self) -> Self {
        self.stuff.unset_first_weapon();
        self
    }

    pub fn drop_second_weapon(mut self) -> Self {
        self.stuff.unset_second_weapon();

        self
    }

    pub fn grab_armor<A: Armor + 'static>(mut self, armor: A) -> Self {
        self.stuff = self.stuff.equip_armor(armor);
        self
    }

    /// Block damage if the set of weapons allow it.
    /// We could have specials skills there to unlock to apply as modified on the amount of blocked damages.
    fn check_blocking_damages(&self) -> Option<BlockedDamages> {
        match self.stuff.get_weapon_settings() {
            StuffConfig::DualWeapons => {
                None // Could have skill to unlock blocking with dual weapon
            }
            StuffConfig::ShieldAndWeapon => self.stuff.get_second_weapon_blocking_damage(),
            StuffConfig::TwoHandsWeapon => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::OnlyShied => self.stuff.get_second_weapon_blocking_damage(),
            StuffConfig::OneSingleHandWeapon => self.stuff.get_first_weapon_blocking_damage(),
            StuffConfig::OneWeaponAsSecondary => None,
        }
    }

    pub fn can_block(&self) -> Option<BlockedDamages> {
        self.check_blocking_damages()
    }

    pub fn get_attacked_by(&mut self, damages: RawDamages, attack_dice: u8, def_dice: Option<u8>) {
        // We could have armor skills to add to the calculation
        let mut receive_damage = damages - self.get_armor();

        if let Some(def_result) = def_dice {
            if def_result > attack_dice {
                let blocking_damage = self.can_block().unwrap_or(0.0);
                receive_damage -= blocking_damage;

                if receive_damage < 0.0 {
                    receive_damage = 0.0;
                }
                println!(
                    "{} blocked {} with its weapon",
                    self.name(),
                    blocking_damage
                )
            } else {
                println!("{} failed blocking the attack ", self.name());
            }
        } else {
            println!("{} Will not block the attack", self.name);
        }
        println!("{} received {} damages", self.name, receive_damage);
        self.health -= receive_damage;

        if self.health < 0.0 {
            self.health = 0.0
        }
    }
    fn get_armor(&self) -> BlockedDamages {
        self.stuff.get_armor_rating()
    }

    /// Get a status to describe the health of the character.
    pub fn get_health_status(&self) -> HealthStatus {
        let percentage: u8 = ((self.health / self.max_health) * 100.0) as u8;
        match percentage {
            0 => HealthStatus::Dead,
            1..=10 => HealthStatus::AlmostDead,
            11..=30 => HealthStatus::SeriouslyHurt,
            31..=50 => HealthStatus::VeryHurt,

            51..=75 => HealthStatus::LightlyHurt,
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

#[derive(PartialEq, Debug)]
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
        Character::new("test character", 1000.0)
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
    fn test_blocking_for_weapon_config() {
        let mut guard = get_test_player().grab_weapon(get_steel_battle_axe());

        // Nice as integration test to check the calculate value is correct
        assert_eq!(
            guard.check_blocking_damages().unwrap(),
            get_steel_battle_axe().damages() * 0.5
        );

        guard = guard.grab_weapon(get_iron_shield());

        assert_eq!(
            guard.check_blocking_damages().unwrap(),
            get_iron_shield().can_block_if_possible().unwrap()
        );

        guard = guard.grab_weapon(get_long_iron_sword());

        // still the same because we put the new weapon on right hand.
        assert_eq!(
            guard.check_blocking_damages().unwrap(),
            get_iron_shield().can_block_if_possible().unwrap()
        );

        // We throw away the shield and go two weapons
        guard = guard.grab_weapon(get_long_iron_sword());
        assert!(guard.check_blocking_damages().is_none());

        guard = guard.drop_second_weapon();
        // One single hand weapon wo we can block
        assert_eq!(
            guard.check_blocking_damages().unwrap(),
            get_long_iron_sword().damages() * 0.3
        );
    }

    #[test]
    fn check_status() {
        let mut guard_test = get_test_player();
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::Healthy);

        guard_test.set_health(850.00);
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::SlightlyHurt);

        guard_test.set_health(550.00);
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::LightlyHurt);

        guard_test.set_health(350.00);
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::VeryHurt);

        guard_test.set_health(250.00);
        assert_eq!(
            &guard_test.get_health_status(),
            &HealthStatus::SeriouslyHurt
        );

        guard_test.set_health(50.00);
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::AlmostDead);

        guard_test.set_health(0.0);
        assert_eq!(&guard_test.get_health_status(), &HealthStatus::Dead);
    }

    #[test]
    fn kill_naked_character() {
        let mut guard = get_test_player();
        guard.get_attacked_by(1800.2, 3, None);
        assert_eq!(&guard.get_health_status(), &HealthStatus::Dead);
    }

    #[test]
    fn defense_armored_character() {
        let mut guard = get_test_player().grab_armor(get_daedric_mail());
        guard.get_attacked_by(100.0, 3, None);
        assert_eq!(guard.health, 945.0);
    }

    #[test]
    fn defense_armored_character_with_block() {
        let mut guard = get_test_player()
            .grab_armor(get_daedric_mail())
            .grab_weapon(get_iron_shield());
        guard.get_attacked_by(100.0, 3, Some(5));

        assert_eq!(guard.health, 970.0);
    }
}
