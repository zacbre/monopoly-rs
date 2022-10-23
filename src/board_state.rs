use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use crate::dice::{RollEngine, DiceRoll};
use crate::location::Properties;
use crate::Player;

pub struct BoardState {
    pub player_turn: usize,
    pub players: HashMap<usize, Player>,
    pub roll_engine: Rc<RefCell<dyn RollEngine>>,
    pub properties: Properties,
}

impl BoardState {
    pub fn new(roll_engine: Rc<RefCell<dyn RollEngine>>) -> Self {
        BoardState {
            player_turn: 1,
            players: HashMap::new(),
            roll_engine,
            properties: Properties::get_new_list()
        }
    }

    pub fn add_players(&mut self, players: Vec<Player>) {
        for player in players.into_iter() {
            self.add_player(player);
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(self.players.len() + 1, player);
    }

    pub fn take_turn(&mut self) {
        let roll = self.roll_engine.borrow().roll();
        let current_player = self.get_current_player();

        current_player.update_location(&roll);
        let player_location = current_player.current_location.clone();
        // get the next property.
        let property = self.properties.get_property(player_location);
        property.run_fn(current_player);

        self.player_turn = self.get_next_player_turn(&roll);
    }

    fn get_current_player_by_number(&mut self, player_number: usize) -> &mut Player {
        match self.players.entry(player_number) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(_) => panic!("Player not found!"),
        }
    }

    fn get_current_player(&mut self) -> &mut Player {
        self.get_current_player_by_number(self.player_turn)
    }

    fn get_next_player_turn(&self, roll: &DiceRoll) -> usize {
        if roll.is_doubles() {
            return self.player_turn;
        }

        self.player_turn % self.players.len() + 1
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::{BoardState, Player};
    use crate::dice::{DiceRoll, RollEngine};
    use crate::location::LocationEnum;

    struct MockContext {
        roll_engine: Rc<RefCell<MockedRollEngine>>,
    }

    struct MockedRollEngine(DiceRoll);
    impl MockedRollEngine {
        pub fn new() -> Self {
            Self {
                0: DiceRoll(1, 1),
            }
        }
        pub fn set_roll(&mut self, first: i32, second: i32) {
            self.0 = DiceRoll(first, second);
        }
    }
    impl RollEngine for MockedRollEngine {
        fn roll(&self) -> DiceRoll {
            return self.0.clone();
        }
    }

    fn get_new_board(player_count: i32) -> (MockContext, BoardState) {
        let mocked_roll_engine = Rc::new(RefCell::new(MockedRollEngine::new()));

        let mock_context = MockContext {
            roll_engine: mocked_roll_engine.clone()
        };

        let mut board = BoardState::new(mocked_roll_engine.clone());
        board.add_players(Player::create_players(player_count));

        (mock_context, board)
    }

    #[test]
    fn can_get_current_player_instance() {
        let (_, mut board) = get_new_board(5);
        let player = board.get_current_player();
        player.cash = 1200;
        let got_player_again = board.get_current_player();
        assert_eq!(1200, got_player_again.cash);
    }

    #[test]
    fn can_get_next_player_turn() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(2, 1);
        board.take_turn();
        assert_eq!(2, board.player_turn);
    }

    #[test]
    fn next_player_turn_is_same_if_rolled_doubles() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(1, 1);
        board.take_turn();
        assert_eq!(1, board.player_turn);
    }

    #[test]
    fn take_turn_player_advances_past_go() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(2, 2);
        let player = board.get_current_player();
        player.current_location = LocationEnum::Boardwalk;
        board.take_turn();
        let player = board.get_current_player();
        assert_eq!(player.current_location, LocationEnum::BalticAvenue);
    }

    #[test]
    fn cannot_leave_jail_without_rolling_doubles() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(2, 3);
        let player = board.get_current_player();
        player.send_to_jail();
        for _ in 0..3 {
            board.take_turn();
            board.player_turn = 1;
            let player = board.get_current_player();
            assert_eq!(player.current_location, LocationEnum::Jail);
        }
        board.take_turn();
        let player = board.get_current_player_by_number(1);
        assert_eq!(player.current_location, LocationEnum::PennsylvaniaRailroad);
        assert_eq!(player.is_in_jail, false);
        assert_eq!(player.doubles_roll_jail_count, 0);
    }

    #[test]
    fn if_player_rolls_a_double_they_can_leave_jail() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(2, 2);
        let player = board.get_current_player();
        player.send_to_jail();
        board.take_turn();
        let player = board.get_current_player_by_number(1);
        assert_eq!(player.current_location, LocationEnum::VirginiaAvenue);
        assert_eq!(player.is_in_jail, false);
        assert_eq!(player.doubles_roll_jail_count, 0);
    }

    #[test]
    fn if_player_rolls_past_go_they_gain_200_cash() {
        let (context, mut board) = get_new_board(5);
        (context.roll_engine.as_ref()).borrow_mut().set_roll(1, 2);
        let player = board.get_current_player();
        player.current_location = LocationEnum::Boardwalk;
        board.take_turn();
        let player = board.get_current_player_by_number(1);
        assert_eq!(player.current_location, LocationEnum::CommunityChest1);
        assert_eq!(player.cash, 1700)
    }

    #[test]
    fn if_player_goes_to_jail_they_do_not_gain_200_cash() {
    }
}