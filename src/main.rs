use std::cell::RefCell;
use std::rc::Rc;
use crate::board_state::BoardState;
use crate::dice::{Dice};
use crate::player::Player;

mod board_state;
mod player;
mod location;
mod cards;
mod dice;

fn main() {

}

fn create_new_game(player_count: i32) -> BoardState {
    let roll_engine = Rc::new(RefCell::new(Dice{}));
    let mut board = BoardState::new(roll_engine);
    board.add_players(create_new_players(player_count));
    board
}

fn create_new_players(player_count: i32) -> Vec<Player> {
    Player::create_players(player_count)
}

fn create_new_player(player_number: i32) -> Player {
    Player::new(player_number as usize)
}

#[cfg(test)]
mod test {
    use crate::{create_new_game, create_new_player, create_new_players};

    #[test]
    fn can_create_new_game() {
        let board = create_new_game(5);
        assert_eq!(5, board.players.len());
        assert_eq!(1, board.player_turn);
    }

    #[test]
    fn can_add_players_to_board() {
        let mut board = create_new_game(0);
        assert_eq!(0, board.players.len());
        board.add_players(create_new_players(2));
        assert_eq!(2, board.players.len());
    }

    #[test]
    fn can_add_player_to_board() {
        let mut board = create_new_game(0);
        assert_eq!(0, board.players.len());
        board.add_player(create_new_player(1));
        assert_eq!(1, board.players.len());
    }
}