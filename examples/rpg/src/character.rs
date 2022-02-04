use crate::item::RegularWeapon;
use crate::item::Weapon;
use crate::item::*;
use crate::stuff::Stuff;

pub struct Character {
    name: String,
    health: f32,
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

enum EquippedWeapons<W1, W2> {
    TwoHand(W1),
    TwoWeapons { right_hand: W1, left_hand: W2 },
    OneSingleRightWeapon { right_hand: W1 },
}

impl<W1, W2> EquippedWeapons<W1, W2>
where
    W1: Default + Weapon + Into<W2>,
    W2: Default + Weapon,
{
    fn new(first_weapon: W1, second_weapons: Option<W2>) -> Self {
        match first_weapon.handheld_type() {
            HandheldType::TwoHands => EquippedWeapons::TwoHand(first_weapon),
            HandheldType::SingleHand => {
                if let Some(left_weapon) = second_weapons {
                    EquippedWeapons::TwoWeapons {
                        right_hand: first_weapon,
                        left_hand: left_weapon,
                    }
                } else {
                    EquippedWeapons::OneSingleRightWeapon {
                        right_hand: first_weapon,
                    }
                }
            }

            HandheldType::OnlyLeft => EquippedWeapons::TwoWeapons {
                right_hand: Default::default(),
                left_hand: first_weapon.into(),
            },
        }
    }

    pub fn two_hands(weapon: W1) -> Self {
        EquippedWeapons::TwoHand(weapon)
    }

    /// Take the actual right weapon and move it to left
    pub fn replace_weapon(actual_weapon: W1, new_weapon: W1) -> Self {
        EquippedWeapons::new(new_weapon, Some(actual_weapon.into()))
    }

    /// Take the actual right weapon and move it to left
    pub fn add_weapon_to_left_hand(actual_weapon: W1, new_weapon: W1) -> Self {
        EquippedWeapons::new(actual_weapon, Some(new_weapon.into()))
    }
}

impl Character {
    pub fn new(name: &str, health: f32) -> Self {
        Character {
            name: name.to_string(),
            health,
            stuff: Default::default(),
        }
    }

    pub fn grab_weapon<W: Weapon + 'static>(mut self, new_weapon: W) -> Self
    where
        W: Weapon,
    {
        match new_weapon.handheld_type() {
            HandheldType::SingleHand => {
                if let Some(first_hand_weapon) = self.stuff.first_weapon() {
                } else {
                }
            }
            HandheldType::OnlyLeft => {}
            HandheldType::TwoHands => {}
        }

        self
    }

    pub fn equip_armor(mut self, armor: BodyArmor) -> Self {
        self.armor = Some(armor);
        self
    }
}
