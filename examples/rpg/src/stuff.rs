use crate::item::{Armor, HandheldType, Weapon};
use std::rc::Rc;

/// https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box
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

    pub fn set_first_weapon<W: 'static + Weapon>(&mut self, first_weapon: W) {
        self.first_weapon = Some(Rc::new(first_weapon));
    }

    pub fn set_second_weapon<W: 'static + Weapon>(&mut self, second_weapon: W) {
        self.second_weapon = Some(Rc::new(second_weapon))
    }

    pub fn unset_first_weapon(mut self) -> Self {
        self.first_weapon = None;
        self
    }

    pub fn unset_second_weapon(mut self) -> Self {
        self.second_weapon = None;
        self
    }

    pub fn armor(&self) -> &Option<Rc<dyn Armor>> {
        &self.armor
    }

    pub fn first_weapon(&self) -> &Option<Rc<dyn Weapon>> {
        &self.first_weapon
    }

    pub fn second_weapon(&self) -> &Option<Rc<dyn Weapon>> {
        &self.second_weapon
    }

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
            HandheldType::OnlyLeft => {}
            HandheldType::TwoHands => {}
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
    use crate::item::{HandheldType, RegularWeapon, Shield};
    use crate::stuff::Stuff;
    use std::rc::Rc;

    fn get_long_iron_sword() -> RegularWeapon {
        RegularWeapon::new("Long Iron Sword", 25.0, HandheldType::SingleHand)
    }

    fn get_steel_battle_exe() -> RegularWeapon {
        RegularWeapon::new("Steal battle Axe", 65.0, HandheldType::TwoHands)
    }

    fn get_iron_shield() -> Shield {
        Shield::new("Iron Shield", 25.0, 5.0)
    }

    fn get_steel_shield() -> Shield {
        Shield::new("Steel Shield", 35.0, 7.0)
    }

    #[test]
    fn get_stuff() {
        let iron_long_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();
        let iron_shield = get_iron_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(iron_long_sword)),
            Some(Rc::new(steel_shield)),
        );

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            "Long Iron Sword"
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            "Steel Shield"
        );

        stuff.set_second_weapon(iron_shield);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            "Long Iron Sword"
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            "Iron Shield"
        );
    }

    #[test]
    fn get_single_hand_weapon() {
        let iron_long_sword = get_long_iron_sword();
        let another_iron_long_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();
        let iron_shield = get_iron_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(iron_long_sword)),
            Some(Rc::new(steel_shield)),
        );
        // because of mut, need &mut self if we want to update partialy the object;
        stuff = stuff.add_weapon(another_iron_long_sword);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            "Long Iron Sword"
        );

        assert_eq!(
            stuff.second_weapon.as_ref().unwrap().name().to_string(),
            "Long Iron Sword"
        );
    }

    #[test]
    fn replace_two_hands_weapon_with_single() {
        let steel_battle_axe = get_steel_battle_exe();
        let mut stuff = Stuff::new(None, Some(Rc::new(steel_battle_axe)), None);
        let iron_long_sword = get_long_iron_sword();
        stuff = stuff.add_weapon(iron_long_sword);
        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            "Long Iron Sword"
        );
        assert!(stuff.second_weapon.is_none());
    }

    fn get_two_hands_weapons() {
        let steel_battle_axe = get_steel_battle_exe();
        let iron_long_sword = get_long_iron_sword();
        let another_iron_long_sword = get_long_iron_sword();
        let steel_shield = get_steel_shield();

        let mut stuff = Stuff::new(
            None,
            Some(Rc::new(iron_long_sword)),
            Some(Rc::new(steel_shield)),
        );

        stuff = stuff.add_weapon(steel_battle_axe);

        assert_eq!(
            stuff.first_weapon.as_ref().unwrap().name().to_string(),
            "Steal battle Axe"
        );

        assert!(stuff.second_weapon.is_none());
    }
}
