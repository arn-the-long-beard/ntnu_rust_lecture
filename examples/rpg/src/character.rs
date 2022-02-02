use crate::item::RegularWeapon;
use crate::item::Weapon;
use crate::item::*;

pub struct Character<W> {
    name: String,
    health: f32,
    armor: Option<BodyArmor>,
    weapons: EquippedWeapons<W>,
}

enum EquippedWeapons<W> {
    TwoHand(W),
    TwoWeapons { right_hand: W, left_hand: W },
    OneSingleRightWeapon { right_hand: W },
}

impl<W: Weapon + Item + Default> EquippedWeapons<W> {
    fn new(first_weapon: W, second_weapons: Option<W>) -> Self {
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
                left_hand: first_weapon,
            },
        }
    }

    pub fn two_hands(weapon: W) -> Self {
        EquippedWeapons::TwoHand(weapon)
    }

    /// Take the actual right weapon and move it to left
    pub fn replace_weapon(actual_weapon: W, new_weapon: W) -> Self {
        EquippedWeapons::new(new_weapon, Some(actual_weapon))
    }
    /// Take the actual right weapon and move it to left
    pub fn add_weapon_to_left_hand(actual_weapon: W, new_weapon: W) -> Self {
        EquippedWeapons::new(actual_weapon, Some(new_weapon))
    }
}

impl<W> Character<W>
where
    W: Weapon + Default + Item,
{
    pub fn new(name: &str, health: f32) -> Self {
        let right_hand = W::default();
        let left_hand = W::default();
        Character {
            name: name.to_string(),
            health,
            armor: None,
            weapons: EquippedWeapons::new(right_hand, Some(left_hand)),
        }
    }

    pub fn equip_weapon(mut self, weapon: W) -> Self {
        self.weapons = match self.weapons {
            EquippedWeapons::TwoHand(_) => EquippedWeapons::two_hands(weapon),
            EquippedWeapons::TwoWeapons { right_hand, .. } => {
                EquippedWeapons::replace_weapon(right_hand, weapon)
            }
            EquippedWeapons::OneSingleRightWeapon { right_hand } => match weapon.handheld_type() {
                HandheldType::SingleHand | HandheldType::OnlyLeft => {
                    EquippedWeapons::add_weapon_to_left_hand(right_hand, weapon)
                }
                HandheldType::TwoHands => EquippedWeapons::two_hands(weapon),
            },
        };
        self
    }

    pub fn equip_armor(mut self, armor: BodyArmor) -> Self {
        self.armor = Some(armor);
        self
    }
}
