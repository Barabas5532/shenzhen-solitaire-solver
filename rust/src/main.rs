use crate::card::{Card, Suit, DRAGON_VALUE};
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
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 6,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 8,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 4,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 3,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 3,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Special,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 6,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 8,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 8,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 3,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
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
                        value: 8,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 3,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 8,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 2,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 3,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Special,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 4,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 3,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 8,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 9,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
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
                        value: 1,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 7,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 4,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 3,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 7,
                    },
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Special,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 9,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 3,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: 3,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 5,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 1,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 7,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 8,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 2,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 8,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Red,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Green,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 9,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 5,
                    },
                ],
                vec![
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Black,
                        value: DRAGON_VALUE,
                    },
                    Card {
                        suit: Suit::Red,
                        value: 6,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 8,
                    },
                    Card {
                        suit: Suit::Black,
                        value: 6,
                    },
                ],
            ],
        },
    ];

    for state in &states {
        println!("{}", state);
    }

    for (i, state) in states.iter().enumerate() {
        let mut game = Game::new(1);
        let solution = game.play(state.clone());

        match solution {
            None => {
                println!("Failed to solve {}", i);
            }
            Some(solution) => {
                //println!("solution {} length {}", i, solution.len());
            }
        }

        // TODO write to file

        /*
        with open(f"solution{i}.txt", "w") as f:
            f.write(f"{solution}")
         */
    }
}
