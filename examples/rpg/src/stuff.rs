use crate::item::*;
use std::rc::Rc;

/// https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box
/// Option on Rc is maybe useless, I could use default value for Weapon & Armor as well.
#[derive(Default)]
pub struct Stuff {
    armor: Option<Rc<dyn Armor>>,
    first_weapon: Option<Rc<dyn Weapon>>,
    second_weapon: Option<Rc<dyn Weapon>>,
}

impl Stuff {
    fn set_armor<A: 'static + Armor>(&mut self, armor: A) {
        self.armor = Some(Rc::new(armor));
    }

    fn set_first_weapon<W: 'static + Weapon>(&mut self, first_weapon: W) {
        self.first_weapon = Some(Rc::new(first_weapon));
    }

    fn set_second_weapon<W: 'static + Weapon>(&mut self, second_weapon: W) {
        self.second_weapon = Some(Rc::new(second_weapon))
    }

    fn unset_first_weapon(&mut self) {
        self.first_weapon = None;
    }

    fn unset_second_weapon(&mut self) {
        self.second_weapon = None;
    }

    fn armor(&self) -> &Option<Rc<dyn Armor>> {
        &self.armor
    }

    fn first_weapon(&self) -> &Option<Rc<dyn Weapon>> {
        &self.first_weapon
    }

    fn second_weapon(&self) -> &Option<Rc<dyn Weapon>> {
        &self.second_weapon
    }
    /// Will panic if you have equipped a two hand weapon as a second Weapon.
    /// We could have specific trait for weapons to be used with both Hands.
    /// Ex : SingleHand Item could have a trait "BothHand", and restrict this trait for second hand.
    ///
    pub fn equip_weapon<W: 'static + Weapon>(mut self, weapon: W) -> Self {
        match weapon.handheld_type() {
            HandheldType::SingleHand => {
                if let Some(current_weapon) = self.first_weapon() {
                    if current_weapon.handheld_type() == &HandheldType::SingleHand {
                        self.second_weapon = Some(current_weapon.clone())
                    }
                }
                self.set_first_weapon(weapon);
            }
            HandheldType::OnlyLeft => {
                if let Some(current_first_weapon) = self.first_weapon() {
                    if current_first_weapon.handheld_type() == &HandheldType::TwoHands {
                        self.unset_first_weapon();
                    }
                }

                // See comment on how we could avoid this issue at compile time.
                // if First weapon is set or not, we do not care, left item always goes left.
                if let Some(current_second_weapon) = self.second_weapon() {
                    if current_second_weapon.handheld_type() == &HandheldType::TwoHands {
                        panic!("It seems you have a two hand weapon as second weapon");
                    }
                }

                self.set_second_weapon(weapon)
            }
            HandheldType::TwoHands => {
                self.unset_second_weapon();
                self.set_first_weapon(weapon);
            }
        }
        self
    }

    ///Calculate how much damage the equipped weapons can do.
    /// Bash damages from shield are counted
    pub fn calculate_damages(&self) -> RawDamages {
        let mut damages: RawDamages = 0.0;

        if let Some(first_weapon) = self.first_weapon() {
            damages = *first_weapon.damages();
        }
        if let Some(second_weapon) = self.second_weapon() {
            damages += *second_weapon.damages();
        }
        damages
    }

    pub fn equip_armor<A: 'static + Armor>(mut self, armor: A) -> Self {
        self.set_armor(armor);
        self
    }

    /// Calculate armor rating.
    pub fn get_armor_rating(&self) -> ArmorRating {
        if let Some(armor) = self.armor() {
            *armor.armor_rating()
        } else {
            0.0
        }
    }

    pub fn calculate_blocked_damage_armor(&self) -> BlockedDamages {
        let armor_damages = if let Some(armor) = self.armor() {
            *armor.armor_rating()
        } else {
            0.0
        };
        armor_damages
    }

    fn is_single_weapon(&self) -> bool {
        if let Some(w) = self.first_weapon() {
            w.handheld_type() == &HandheldType::SingleHand && self.second_weapon().is_none()
        } else {
            false
        }
    }

    fn is_double_weapon(&self) -> bool {
        if let (Some(first_weapon), Some(second_weapon)) =
            (self.first_weapon(), self.second_weapon())
        {
            first_weapon.handheld_type() == &HandheldType::SingleHand
                && second_weapon.handheld_type() == &HandheldType::SingleHand
        } else {
            false
        }
    }

    fn is_two_hands_weapon(&self) -> bool {
        if let (Some(first_weapon), None) = (self.first_weapon(), self.second_weapon()) {
            first_weapon.handheld_type() == &HandheldType::TwoHands
        } else {
            false
        }
    }

    fn is_shield_with_single_weapon(&self) -> bool {
        if let (Some(first_weapon), Some(second_weapon)) =
            (self.first_weapon(), self.second_weapon())
        {
            first_weapon.handheld_type() == &HandheldType::SingleHand
                && second_weapon.handheld_type() == &HandheldType::OnlyLeft
        } else {
            false
        }
    }

    fn is_shield_only(&self) -> bool {
        if let (None, Some(second_weapon)) = (self.first_weapon(), self.second_weapon()) {
            second_weapon.handheld_type() == &HandheldType::OnlyLeft
        } else {
            false
        }
    }

    fn is_one_single_as_secondary(&self) -> bool {
        if let (None, Some(second_weapon)) = (self.first_weapon(), self.second_weapon()) {
            second_weapon.handheld_type() == &HandheldType::SingleHand
        } else {
            false
        }
    }

    pub fn get_weapon_settings(&self) -> StuffConfig {
        if self.is_double_weapon() {
            StuffConfig::DualWeapons
        } else if self.is_shield_only() {
            StuffConfig::OnlyShied
        } else if self.is_shield_with_single_weapon() {
            StuffConfig::ShieldAndWeapon
        } else if self.is_single_weapon() {
            StuffConfig::OneSingleHandWeapon
        } else if self.is_two_hands_weapon() {
            StuffConfig::TwoHandsWeapon
        } else if self.is_one_single_as_secondary() {
            StuffConfig::OneWeaponAsSecondary
        } else {
            panic!("Config not found maybe no weapons have been equipped")
        }
    }

    pub fn get_first_weapon_blocking_damage(&self) -> Option<f32> {
        self.first_weapon.as_ref()?.can_block_if_possible()
    }

    pub fn get_second_weapon_blocking_damage(&self) -> Option<f32> {
        self.second_weapon.as_ref()?.can_block_if_possible()
    }
}

#[derive(PartialEq)]
pub enum StuffConfig {
    DualWeapons,
    ShieldAndWeapon,
    TwoHandsWeapon,
    OnlyShied,
    OneSingleHandWeapon,
    OneWeaponAsSecondary,
}

#[cfg(test)]
mod test {
    use crate::item::{Armor, BodyArmor, HandheldType, Item, RegularWeapon, Shield, Weapon};
    use crate::stuff::{Stuff, StuffConfig};

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
    fn use_single_hand_weapon_in_place_of_shield() {
        let long_iron_sword = get_long_iron_sword();
        let another_long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::default()
            .equip_weapon(long_iron_sword)
            .equip_weapon(steel_shield);

        // because of mut, need &mut self if we want to update partialy the object;
        stuff = stuff.equip_weapon(another_long_iron_sword);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );
        assert!(stuff.get_weapon_settings() == StuffConfig::DualWeapons);
    }

    #[test]
    /// Let's see when we move the first weapon as second
    /// Let's see if using just one weapon works
    fn replace_single_hand_weapons() {
        let long_iron_sword = get_long_iron_sword();
        let long_steel_sword = get_long_steel_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::default()
            .equip_weapon(long_iron_sword)
            .equip_weapon(steel_shield);

        stuff = stuff.equip_weapon(long_steel_sword);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_steel_sword().name()
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );
    }

    #[test]
    fn replace_two_hands_weapon_with_single() {
        let steel_battle_axe = get_steel_battle_axe();
        let mut stuff = Stuff::default().equip_weapon(steel_battle_axe);
        let long_iron_sword = get_long_iron_sword();
        stuff = stuff.equip_weapon(long_iron_sword);
        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );
        assert!(stuff.second_weapon.is_none());

        assert!(stuff.get_weapon_settings() == StuffConfig::OneSingleHandWeapon);
    }

    #[test]
    fn use_two_hands_weapons() {
        let steel_battle_axe = get_steel_battle_axe();
        let long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();
        let mut stuff = Stuff::default()
            .equip_weapon(long_iron_sword)
            .equip_weapon(steel_shield);

        stuff = stuff.equip_weapon(steel_battle_axe);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_steel_battle_axe().name()
        );

        assert!(stuff.second_weapon.is_none());
        assert!(stuff.get_weapon_settings() == StuffConfig::TwoHandsWeapon);
    }

    #[test]
    fn use_shields() {
        let long_iron_sword = get_long_iron_sword();
        let another_long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();
        let iron_shield = get_iron_shield();

        let mut stuff = Stuff::default()
            .equip_weapon(long_iron_sword)
            .equip_weapon(steel_shield);

        stuff = stuff.equip_weapon(iron_shield);

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_iron_shield().name()
        );

        stuff = stuff
            .equip_weapon(get_iron_shield())
            .equip_weapon(get_steel_shield());

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_steel_shield().name()
        );
        assert!(stuff.get_weapon_settings() == StuffConfig::ShieldAndWeapon);
    }

    #[test]
    fn test_damages_with_shield_and_sword() {
        let damages = Stuff::default()
            .equip_weapon(get_long_steel_sword())
            .equip_weapon(get_steel_shield())
            .calculate_damages();

        assert_eq!(
            damages,
            get_steel_shield().damages() + get_long_steel_sword().damages()
        )
    }

    #[test]
    fn test_damages_with_shield() {
        let damages = Stuff::default()
            .equip_weapon(get_steel_shield())
            .calculate_damages();
        assert_eq!(&damages, get_steel_shield().damages())
    }

    #[test]
    fn test_damages_with_two_hands() {
        let damages = Stuff::default()
            .equip_weapon(get_steel_battle_axe())
            .calculate_damages();
        assert_eq!(&damages, get_steel_battle_axe().damages())
    }

    #[test]
    fn test_armor_rating_with_no_armor() {
        let rating = Stuff::default().get_armor_rating();

        assert_eq!(rating, 0.0)
    }

    #[test]
    fn test_armor_rating_with_armor() {
        let rating = Stuff::default()
            .equip_armor(get_daedric_mail())
            .get_armor_rating();

        assert_eq!(&rating, get_daedric_mail().armor_rating());
    }
}
