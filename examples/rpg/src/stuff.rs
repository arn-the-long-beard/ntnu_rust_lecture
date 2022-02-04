use crate::item::{Armor, HandheldType, Weapon};
use std::rc::Rc;

/// https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box
/// Option on Rc is maybe useless, I could use default value for Weapon as well.
#[derive(Default)]
pub struct Stuff {
    armor: Option<Rc<dyn Armor>>,
    first_weapon: Option<Rc<dyn Weapon>>,
    second_weapon: Option<Rc<dyn Weapon>>,
}

impl Stuff {
    pub fn set_armor(&mut self, armor: Option<Rc<dyn Armor>>) {
        self.armor = armor;
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

    pub fn armor(&self) -> &Option<Rc<dyn Armor>> {
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
    pub fn add_weapon<W: 'static + Weapon>(mut self, weapon: W) -> Self {
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
}

impl Stuff {
    pub fn new(
        armor: Option<Rc<dyn Armor>>,
        first_weapon: Option<Rc<dyn Weapon>>,
        second_weapon: Option<Rc<dyn Weapon>>,
    ) -> Self {
        Stuff {
            armor,
            first_weapon,
            second_weapon,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::item::{HandheldType, Item, RegularWeapon, Shield};
    use crate::stuff::Stuff;
    use std::rc::Rc;

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

    #[test]
    /// Let's see when we move the first weapon as second
    /// Let's see if using just one weapon works
    fn use_single_hand_weapon() {
        let long_iron_sword = get_long_iron_sword();
        let long_steel_sword = get_long_steel_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(long_iron_sword)),
            Some(Rc::new(steel_shield)),
        );

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_steel_shield().name()
        );

        stuff = stuff.add_weapon(long_steel_sword);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_steel_sword().name()
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );

        stuff.unset_first_weapon();

        assert!(stuff.first_weapon.is_none());

        let sword = get_long_steel_sword();
        stuff = stuff.add_weapon(sword);

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
    fn get_single_hand_weapon() {
        let long_iron_sword = get_long_iron_sword();
        let another_long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(long_iron_sword)),
            Some(Rc::new(steel_shield)),
        );
        // because of mut, need &mut self if we want to update partialy the object;
        stuff = stuff.add_weapon(another_long_iron_sword);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );
    }

    #[test]
    fn replace_two_hands_weapon_with_single() {
        let steel_battle_axe = get_steel_battle_axe();
        let mut stuff = Stuff::new(None, Some(Rc::new(steel_battle_axe)), None);
        let long_iron_sword = get_long_iron_sword();
        stuff = stuff.add_weapon(long_iron_sword);
        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_long_iron_sword().name()
        );
        assert!(stuff.second_weapon.is_none());
    }

    #[test]
    fn get_two_hands_weapons() {
        let steel_battle_axe = get_steel_battle_axe();
        let long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(long_iron_sword)),
            Some(Rc::new(steel_shield)),
        );

        stuff = stuff.add_weapon(steel_battle_axe);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            get_steel_battle_axe().name()
        );

        assert!(stuff.second_weapon.is_none());
    }

    #[test]
    fn get_shields() {
        let long_iron_sword = get_long_iron_sword();
        let another_long_iron_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();
        let iron_shield = get_iron_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(long_iron_sword)),
            Some(Rc::new(steel_shield)),
        );

        stuff = stuff.add_weapon(iron_shield);

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_iron_shield().name()
        );

        stuff = stuff
            .add_weapon(get_iron_shield())
            .add_weapon(get_steel_shield());

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            get_steel_shield().name()
        );
    }
}
