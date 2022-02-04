//! Using https://doc.rust-lang.org/book/ch17-02-trait-objects.html

pub type BlockedDamages = f32;
pub type RawDamages = f32;
pub type ArmorRating = f32;

//
pub trait Item {
    fn name(&self) -> &str;
    fn set_name(self, name: &str) -> Self
    where
        Self: Sized;
}

// Let's start with armor;
pub trait Armor: Item {
    fn set_armor_rating(self, armor_rating: ArmorRating) -> Self
    where
        Self: Sized;
    fn armor_rating(&self) -> &ArmorRating;
}

/// NB : We could define many different armor type.
pub struct BodyArmor {
    armor_rating: f32,
    name: String,
}

impl Default for BodyArmor {
    fn default() -> Self {
        BodyArmor {
            armor_rating: 0 as f32,
            name: "".to_string(),
        }
    }
}

impl BodyArmor {
    pub fn new(name: &str, armor_rating: f32) -> Self {
        Self::default()
            .set_name(name)
            .set_armor_rating(armor_rating)
    }
}

impl Armor for BodyArmor {
    fn set_armor_rating(mut self, reduction: f32) -> Self {
        self.armor_rating = reduction;
        self
    }
    fn armor_rating(&self) -> &f32 {
        &self.armor_rating
    }
}

impl Item for BodyArmor {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

pub trait Weapon: Item {
    /// Describe how much damage a weapon can deal.
    /// More damage a weapon deals, better quality it is .
    fn damages(&self) -> &RawDamages;
    fn set_damages(self, amount: RawDamages) -> Self
    where
        Self: Sized;
    // Block attack and make calculation if possible
    fn can_block_if_possible(&self) -> Option<BlockedDamages> {
        match self.handheld_type() {
            HandheldType::SingleHand => Some(self.damages() * 0.4),
            HandheldType::TwoHands => Some(self.damages() * 0.7),
            // A bit dummy here because we have different implementation later.
            HandheldType::OnlyLeft => None,
        }
    }
    fn set_handheld_type(self, handheld: HandheldType) -> Self
    where
        Self: Sized;
    fn handheld_type(&self) -> &HandheldType;
}

// NB: I could have made multiple trait instead of enum as well.
#[derive(PartialEq)]
pub enum HandheldType {
    SingleHand,
    OnlyLeft,
    TwoHands,
}

/// NB : We could define many different of weapon ( like enchanted weapons for example and or melee weapons )
pub struct RegularWeapon {
    name: String,
    handheld: HandheldType,
    damages: RawDamages,
}

impl RegularWeapon {
    pub fn new(name: &str, damages: f32, handheld: HandheldType) -> Self {
        RegularWeapon::default()
            .set_name(name)
            .set_damages(damages)
            .set_handheld_type(handheld)
    }
}

impl Default for RegularWeapon {
    fn default() -> Self {
        RegularWeapon {
            name: "Hands".to_string(),
            handheld: HandheldType::SingleHand,
            damages: 1.0,
        }
    }
}

impl Item for RegularWeapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Weapon for RegularWeapon {
    fn damages(&self) -> &RawDamages {
        &self.damages
    }

    fn set_damages(mut self, amount: RawDamages) -> Self {
        self.damages = amount;
        self
    }

    fn set_handheld_type(mut self, handheld: HandheldType) -> Self {
        self.handheld = handheld;
        self
    }

    fn handheld_type(&self) -> &HandheldType {
        &self.handheld
    }
}

#[allow(unused)]
/// NB : Shield could be a trait instead of a struct as well.
pub struct Shield {
    armor_rating: f32,
    name: String,
    hold: HandheldType,
    bash_damage: RawDamages,
    handheld: HandheldType,
}

impl Default for Shield {
    fn default() -> Self {
        Shield {
            armor_rating: 0.0,
            name: "".to_string(),
            hold: HandheldType::OnlyLeft,
            bash_damage: 0.0,
            handheld: HandheldType::OnlyLeft,
        }
    }
}

impl Armor for Shield {
    fn set_armor_rating(mut self, reduction: f32) -> Self {
        self.armor_rating = reduction;
        self
    }

    fn armor_rating(&self) -> &f32 {
        &self.armor_rating
    }
}

impl Item for Shield {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Weapon for Shield {
    fn damages(&self) -> &RawDamages {
        &self.bash_damage
    }

    fn set_damages(mut self, amount: RawDamages) -> Self {
        self.bash_damage = amount;
        self
    }

    fn can_block_if_possible(&self) -> Option<BlockedDamages> {
        //We could have skills here to help us to calculate
        Some(self.armor_rating)
    }

    fn set_handheld_type(mut self, handheld: HandheldType) -> Self {
        self.handheld = handheld;
        self
    }

    fn handheld_type(&self) -> &HandheldType {
        &self.handheld
    }
}

impl Shield {
    pub fn new(name: &str, armor: f32, bash_damages: f32) -> Self {
        Self::default()
            .set_name(name)
            .set_armor_rating(armor)
            .set_handheld_type(HandheldType::OnlyLeft)
            .set_damages(bash_damages)
    }
}
