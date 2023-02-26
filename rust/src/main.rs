use crate::card::{Card, Suit};
use crate::game::{Game, GameMove};
use crate::game_state::GameState;

mod card;
mod game;
mod game_state;

fn main() {
    let states = [
        GameState {
            top_left_storage: vec![],
            top_right_storage: [0, 0, 0, 0],
            columns: [
                vec![
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(6),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(4),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(3),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(3),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Special,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(6),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(8),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(3),
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
            ],
        },
        GameState {
            top_left_storage: vec![],
            top_right_storage: [0, 0, 0, 0],
            columns: [
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(3),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(2),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(3),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Special,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(4),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(3),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(8),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(9),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
            ],
        },
        GameState {
            top_left_storage: vec![],
            top_right_storage: [0, 0, 0, 0],
            columns: [
                vec![
                    Card {
                        suit: Suit::Black,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(7),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(3),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(7),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Special,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(9),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(3),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: Some(3),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(1),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(7),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(2),
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(8),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Green,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(9),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(6),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(8),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(6),
                    },
                ],
            ],
        },
    ];

    for state in &states {
        println!("{}", state);
    }

    for (i, state) in states.iter().enumerate() {
        let mut game = Game::new();
        let solution = game.play(state.clone());

        match solution {
            None => {
                println!("Failed to solve {}", i);
            }
            Some(solution) => {
                println!("solution {} length {}", i, solution.len());
            }
        }

        // TODO write to file

        /*
        with open(f"solution{i}.txt", "w") as f:
            f.write(f"{solution}")
         */
    }
}
