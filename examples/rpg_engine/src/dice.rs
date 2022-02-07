use caith::Roller;
use colored::*;

pub fn roll_dice(actor: &str, dices: &str, action: &str) -> i64 {
    let result = Roller::new(&format!("{} : {} ", dices, action))
        .unwrap()
        .roll()
        .unwrap();
    println!(
        "{} rolls {} for {} ",
        actor.bold(),
        dices.underline(),
        action.magenta()
    );
    result.as_single().unwrap().get_total()
}

pub enum SkillDice {
    Initiative,
    Blocking,
    Attack,
    Dodge,
}

impl SkillDice {
    pub fn dices_roll_result(&self, actor: &str) -> u8 {
        let res = match self {
            SkillDice::Initiative => roll_dice(actor, "2d6", "initiative"),
            SkillDice::Blocking => roll_dice(actor, "1d6", "block"),
            SkillDice::Attack => roll_dice(actor, "1d10", "attack"),
            SkillDice::Dodge => roll_dice(actor, "1d10", "dodge"),
        };

        res as u8
    }
}
