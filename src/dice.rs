use rand::Rng;

pub trait RollEngine {
    fn roll(&self) -> DiceRoll;
}

#[derive(Clone)]
pub struct DiceRoll(pub i32, pub i32);
impl DiceRoll {
    pub fn is_doubles(&self) -> bool {
        self.0 == self.1
    }
}

pub struct Dice;
impl RollEngine for Dice {
    fn roll(&self) -> DiceRoll {
        let mut rng = rand::thread_rng();
        let first = rng.gen_range(1..6);
        let second = rng.gen_range(1..6);

        return DiceRoll{ 0: first, 1: second }
    }
}