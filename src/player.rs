use crate::dice::DiceRoll;
use crate::location::LocationEnum;

pub struct Player {
    pub player_number: usize,
    pub cash: i32,
    pub current_location: LocationEnum,
    pub is_in_jail: bool,
    pub doubles_roll_count: i32,
    pub doubles_roll_jail_count: i32,
}

impl Player {
    pub fn new(player_number: usize) -> Self {
        Player {
            player_number,
            cash: 1500,
            current_location: LocationEnum::Go,
            is_in_jail: false,
            doubles_roll_count: 0,
            doubles_roll_jail_count: 0
        }
    }

    pub fn create_players(player_count: i32) -> Vec<Player> {
        let mut players: Vec<Player> = Vec::new();
        for i in 1..player_count + 1 {
            players.push(Player::new(i as usize));
        }

        players
    }

    pub fn update_location(&mut self, roll: &DiceRoll) {
        if self.is_in_jail {
            if self.doubles_roll_jail_count < 3 && !roll.is_doubles() {
                self.doubles_roll_jail_count += 1;
                return
            } else {
                self.remove_from_jail();
            }
        }

        let last_location = self.current_location;
        self.current_location = LocationEnum::from(((last_location as i32 + roll.0 + roll.1) % 40) as usize);

        self.check_if_passed_go(last_location, self.current_location);
    }

    pub fn send_to_jail(&mut self) {
        self.current_location = LocationEnum::Jail;
        self.is_in_jail = true;
    }

    pub fn remove_from_jail(&mut self) {
        self.is_in_jail = false;
        self.doubles_roll_jail_count = 0;
    }

    pub fn check_if_passed_go(&mut self, old_location: LocationEnum, new_location: LocationEnum) {
        if new_location < old_location {
            self.cash += 200;
        }
    }
}