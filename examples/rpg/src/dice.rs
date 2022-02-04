use caith::Roller;
//
pub fn roll_dice(dices: &str, action: &str) -> i64 {
    let result = Roller::new(&format!("{} : {} ", dices, action))
        .unwrap()
        .roll()
        .unwrap();
    result.as_single().unwrap().get_total()
}

pub enum SkillDice {
    Initiative,
    Blocking,
    Attack,
    Dodge,
}

impl SkillDice {
    pub fn dices_roll_result(&self) -> u8 {
        let res = match self {
            SkillDice::Initiative => roll_dice("1d6", "initiative"),
            SkillDice::Blocking => roll_dice("1d10", "blockinge"),
            SkillDice::Attack => roll_dice("1d10", "attack"),
            SkillDice::Dodge => roll_dice("1d10", "dodge"),
        };

        res as u8
    }
}
