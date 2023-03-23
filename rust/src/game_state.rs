use crate::card::*;
use serde::{Deserialize, Serialize};
use std::fmt::{write, Formatter};
use std::hash::{Hash, Hasher};
use std::{cmp, fmt};

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct GameState {
    // scratch pad to temporarily store cards
    // a space is lost when dragons are stacked here, represented by a
    // Card(Suit.FACE_DOWN, None)
    pub top_left_storage: Vec<Card>,

    // The aim of the game is to get all the cards stacked here
    pub top_right_storage: [u8; 4],

    // The main play area, where all of the cards are placed at the start
    pub columns: [Vec<Card>; 8],
}

pub struct MoveColumnParameters {
    pub from_column_index: usize,
    pub to_column_index: usize,
    pub stack_size: usize,
}

impl GameState {
    // All the columns in the centre have no cards
    pub fn is_solved(&self) -> bool {
        for column in &self.columns {
            if !column.is_empty() {
                return false;
            }
        }

        // this is only possible if all the dragons have been collected, and the
        // top right cards all have value 9
        // This is just a sanity check to make sure the game has not in an
        // invalid state
        assert_eq!(self.top_right_storage.len(), 4);
        assert_eq!(self.top_right_storage[0], 1);
        for i in 1..4 {
            assert_eq!(self.top_right_storage[i], 9);
        }
        assert_eq!(self.top_left_storage.len(), 3);
        for i in 0..3 {
            assert_eq!(self.top_left_storage[i].suit, Suit::FaceDown)
        }

        true
    }

    // TODO Suit.SPECIAL card can always be moved to storage, it's hardcoded to
    // have value of 1 for now
    pub fn can_move_column_to_top_right_storage(&self, column_index: usize) -> bool {
        if self.columns[column_index].is_empty() {
            return false;
        }

        let card = self.columns[column_index]
            .last()
            .expect("Size was already check to be non-zero");

        match { card.value } {
            None => false,
            Some(value) => value == 1 || self.top_right_storage[card.suit as usize] == value - 1,
        }
    }

    pub fn move_column_to_top_right_storage(&mut self, column_index: usize) {
        let card = self.columns[column_index].pop().unwrap();
        self.top_right_storage[card.suit as usize] = card.value.unwrap()
    }

    pub fn can_move_top_left_to_top_right_storage(&self, top_left_index: usize) -> bool {
        if self.top_left_storage.len() <= top_left_index {
            return false;
        }

        let card = &self.top_left_storage[top_left_index];

        match { card.value } {
            None => false,
            Some(value) => self.top_right_storage[card.suit as usize] == value - 1,
        }
    }

    pub fn move_top_left_to_top_right_storage(&mut self, top_left_index: usize) {
        let card = self.top_left_storage.remove(top_left_index);
        self.top_right_storage[card.suit as usize] = card
            .value
            .expect("must call can_move_to_top_right_storage first");
    }

    pub fn can_move_top_left_to_column(&self, top_left_index: usize, column_index: usize) -> bool {
        if top_left_index >= self.top_left_storage.len() {
            return false;
        }

        let card_to_move = &self.top_left_storage[top_left_index];
        // Can't move collected dragons
        if card_to_move.suit == Suit::FaceDown {
            return false;
        }

        // We are sure that the top left storage has a movable card now

        // moving to an empty column is always allowed
        if self.columns[column_index].is_empty() {
            return true;
        }

        let target_card = self.columns[column_index].last().unwrap();
        if target_card.is_dragon() {
            return false;
        }

        card_to_move.can_be_moved_on_top_of(target_card)
    }

    pub fn move_top_left_to_column(&mut self, top_left_index: usize, column_index: usize) {
        self.columns[column_index].push(self.top_left_storage.remove(top_left_index))
    }

    pub fn can_move_column_to_top_left(&self, column_index: usize) -> bool {
        !self.columns[column_index].is_empty() && self.top_left_storage.len() < 3
    }

    pub fn move_column_to_top_left(&mut self, column_index: usize) {
        self.top_left_storage
            .push(self.columns[column_index].pop().unwrap());
        assert!(self.top_left_storage.len() <= 3);
    }

    pub fn can_collect_dragons(&self, suit: Suit) -> bool {
        if 3 == self
            .top_left_storage
            .iter()
            .filter(|card| !card.is_dragon_with_suit(suit))
            .count()
        {
            return false;
        }

        let mut free_dragon_count = 0;
        for column in &self.columns {
            if column.is_empty() {
                continue;
            }

            if column.last().unwrap().is_dragon_with_suit(suit) {
                free_dragon_count += 1;
            }
        }

        for card in &self.top_left_storage {
            if card.is_dragon_with_suit(suit) {
                free_dragon_count += 1
            }
        }

        free_dragon_count == 4
    }

    pub fn collect_dragons(&mut self, suit: Suit) {
        // This is always called after checking if this move is valid.
        // Therefore, we can just remove all the dragons and add a face down
        // card to the top left.
        for column in &mut self.columns {
            column.retain(|card| !card.is_dragon_with_suit(suit));
        }

        self.top_left_storage
            .retain(|card| !card.is_dragon_with_suit(suit));

        self.top_left_storage.push(Card {
            suit: Suit::FaceDown,
            value: None,
        });
        assert!(self.top_left_storage.len() <= 3)
    }

    fn get_column_stack_size(&self, column_index: usize) -> usize {
        if self.columns[column_index].is_empty() {
            return 0;
        }

        let mut stack_size = 1;

        let column = &self.columns[column_index];
        for (i, card) in column.iter().enumerate() {
            if i + 1 == column.len() {
                break;
            }

            let next_card = &column[i + 1];

            // TODO doing this loop from the back might get better performance.
            // We can stop at the first card that is not part of the stack
            // instead of checking all the cards.
            if next_card.can_be_moved_on_top_of(card) {
                stack_size += 1;
            } else {
                stack_size = 1;
            }
        }

        stack_size
    }

    pub fn can_move_column_to_other_column(&self, p: MoveColumnParameters) -> bool {
        let actual_stack_size = self.get_column_stack_size(p.from_column_index);

        // TODO this statement is redundant, stack size is always greater than
        //      zero. Remove once we have enough test coverage
        if actual_stack_size == 0 {
            return false;
        }

        if p.stack_size > actual_stack_size {
            return false;
        }

        if self.columns[p.to_column_index].is_empty() {
            return true;
        }

        let column = &self.columns[p.from_column_index];
        let stack_first_card = &column[column.len() - p.stack_size];
        let target_card = self.columns[p.to_column_index].last().unwrap();
        stack_first_card.can_be_moved_on_top_of(target_card)
    }

    pub fn move_column_to_other_column(&mut self, p: MoveColumnParameters) {
        assert_ne!(p.from_column_index, p.to_column_index);
        let mid = cmp::max(p.from_column_index, p.to_column_index);
        let (left, right) = self.columns.split_at_mut(mid);

        let (from_column, to_column) = if p.from_column_index < mid {
            (
                &mut left[p.from_column_index],
                &mut right[p.to_column_index - mid],
            )
        } else {
            (
                &mut right[p.from_column_index - mid],
                &mut left[p.to_column_index],
            )
        };

        let card_stack = { from_column.drain(from_column.len() - p.stack_size..) };
        to_column.extend(card_stack);
    }
}

impl PartialEq<Self> for GameState {
    fn eq(&self, other: &Self) -> bool {
        let mut top_left_storage = self.top_left_storage.clone();
        top_left_storage.sort();
        let mut other_top_left_storage = self.top_left_storage.clone();
        other_top_left_storage.sort();

        // columns are sorted by the bottom card to try to prevent useless
        // moves moving stacks to another empty column
        let get_bottom_card = |col: &Vec<Card>| {
            *col.first().unwrap_or(&Card {
                suit: Suit::Special,
                value: None,
            })
        };
        let bottom_card_sort =
            |a: &Vec<Card>, b: &Vec<Card>| get_bottom_card(a).cmp(&get_bottom_card(b));
        let mut columns = self.columns.clone();
        columns.sort_by(bottom_card_sort);
        let mut other_columns = other.columns.clone();
        other_columns.sort_by(bottom_card_sort);

        top_left_storage == other_top_left_storage
            && columns == other_columns
            && self.top_right_storage == other.top_right_storage
    }
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut top_left_storage = self.top_left_storage.clone();
        top_left_storage.sort();

        // columns are sorted by the bottom card to try to prevent useless
        // moves moving stacks to another empty column
        let get_bottom_card = |col: &Vec<Card>| {
            *col.first().unwrap_or(&Card {
                suit: Suit::Special,
                value: None,
            })
        };
        let bottom_card_sort =
            |a: &Vec<Card>, b: &Vec<Card>| get_bottom_card(a).cmp(&get_bottom_card(b));
        let mut columns = self.columns.clone();
        columns.sort_by(bottom_card_sort);

        top_left_storage.hash(state);
        self.top_right_storage.hash(state);
        columns.hash(state);
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut top_row = String::from("========== GAME STATE =========\n");

        let mut top_left_sorted = self.top_left_storage.clone();
        top_left_sorted.sort();

        for (_, card) in top_left_sorted.iter().enumerate() {
            write(&mut top_row, format_args!("{} ", card))?;
        }

        for _ in 0..3 - self.top_left_storage.len() + 1 {
            top_row += "    ";
        }

        for (suit, value) in self.top_right_storage.iter().enumerate() {
            let value = *value;

            if value == 0 {
                top_row += "   "
            } else {
                write(
                    &mut top_row,
                    format_args!(
                        "{} ",
                        Card {
                            suit: Suit::try_from(suit).unwrap(),
                            value: Some(value)
                        }
                    ),
                )?;
            }
        }

        let mut columns = String::new();

        let longest_row_len = self.columns.iter().fold(0, |max, v| cmp::max(max, v.len()));

        for row_index in 0..longest_row_len {
            for column in self.columns.iter() {
                let card = column.get(row_index);
                match card {
                    None => {
                        columns += "   ";
                    }
                    Some(card) => {
                        columns += card.to_string().as_str();
                        columns += " ";
                    }
                }
            }
            columns += "\n";
        }

        top_row += "\n";
        top_row += columns.as_str();
        f.write_str(top_row.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Suit::{Black, FaceDown, Green, Red, Special};
    use googletest::matchers::*;
    use googletest::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_move_to_top_right() {
        let mut result = vec![GameState {
            columns: [
                vec![
                    Card {
                        suit: Red,
                        value: Some(9),
                    },
                    Card {
                        suit: Green,
                        value: Some(9),
                    },
                    Card {
                        suit: Black,
                        value: Some(9),
                    },
                    Card {
                        suit: Special,
                        value: Some(1),
                    },
                ],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
            ],
            top_right_storage: [0, 8, 8, 8],
        }];

        for _ in 0..4 {
            let mut state = result.last().unwrap().clone();
            assert_that!(state.is_solved(), eq(false));
            assert_that!(state.can_move_column_to_top_right_storage(0), eq(true));
            state.move_column_to_top_right_storage(0);
            result.push(state);
        }

        assert_that!(result.last().unwrap().is_solved(), eq(true));
    }

    #[test]
    fn test_move_storage_to_top_right() {
        let mut result = vec![GameState {
            columns: [
                vec![Card {
                    suit: Red,
                    value: Some(8),
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![],
                vec![],
                vec![],
            ],
            top_left_storage: vec![
                Card {
                    suit: Red,
                    value: Some(9),
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
            ],
            top_right_storage: [1, 7, 9, 9],
        }];

        for i in 0..2 {
            let mut state = result.last().unwrap().clone();
            assert_that!(state.is_solved(), eq(false));

            if i == 0 {
                assert_that!(state.can_move_column_to_top_right_storage(0), eq(true));
                state.move_column_to_top_right_storage(0);
            } else {
                assert_that!(state.can_move_top_left_to_top_right_storage(0), eq(true));
                state.move_top_left_to_top_right_storage(0);
            }

            result.push(state);
        }

        for s in result {
            println!("{s}")
        }
    }

    #[test]
    fn test_hashable() {
        let empty_columns: [Vec<Card>; 8] = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let mut a = empty_columns.clone();

        let mut b = empty_columns;
        b[1].push(Card {
            suit: Red,
            value: Some(1),
        });

        let c = b.clone();

        let d = a.clone();
        a[0].push(Card {
            suit: Red,
            value: Some(1),
        });

        let state_a = GameState {
            columns: a,
            top_right_storage: [0; 4],
            top_left_storage: vec![],
        };
        let state_b = GameState {
            columns: b,
            top_right_storage: [0; 4],
            top_left_storage: vec![],
        };
        let state_c = GameState {
            columns: c,
            top_right_storage: [0; 4],
            top_left_storage: vec![],
        };
        let state_d = GameState {
            columns: d,
            top_right_storage: [0; 4],
            top_left_storage: vec![],
        };

        let hash_a = calculate_hash(&state_a);
        let hash_b = calculate_hash(&state_b);
        let hash_c = calculate_hash(&state_c);
        let hash_d = calculate_hash(&state_d);

        // Permutations of columns should not effect equality and hash
        assert_that!(state_a, eq(state_b.clone()));
        assert_that!(hash_a, eq(hash_b));

        assert_that!(state_b, eq(state_c));
        assert_that!(hash_b, eq(hash_c));

        assert_that!(state_a, not(eq(state_d)));
        assert_that!(hash_a, not(eq(hash_d)));
    }

    #[test]
    fn test_hash_ignores_top_left_permutation() {
        /* We should be able to optimize the execution time by detecting
         * more identical cycles where the only difference is the permutation
         * of the cards in the top left corner
         *
         * E.G. a position with the top left storage having
         *
         * 🟥x ⬛x 🟩3
         *
         * or
         *
         * 🟩3 🟥x ⬛x
         *
         * is identical from a gameplay perspective
         */
        let empty_columns = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let a = GameState {
            columns: empty_columns.clone(),
            top_left_storage: vec![
                Card {
                    suit: Red,
                    value: None,
                },
                Card {
                    suit: Black,
                    value: None,
                },
                Card {
                    suit: Green,
                    value: Some(3),
                },
            ],
            top_right_storage: [0; 4],
        };

        let b = GameState {
            columns: empty_columns,
            top_left_storage: vec![
                Card {
                    suit: Green,
                    value: Some(3),
                },
                Card {
                    suit: Red,
                    value: None,
                },
                Card {
                    suit: Black,
                    value: None,
                },
            ],
            top_right_storage: [0; 4],
        };

        assert_that!(&a, eq(&b));
        assert_that!(calculate_hash(&a), eq(calculate_hash(&b)));
    }

    #[test]
    fn test_can_move_top_left_to_column() {
        let mut state = GameState {
            columns: [
                vec![
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                ],
                vec![Card {
                    suit: Red,
                    value: Some(9),
                }],
                vec![Card {
                    suit: Green,
                    value: Some(9),
                }],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: Green,
                    value: None,
                },
                Card {
                    suit: Red,
                    value: Some(8),
                },
            ],
            top_right_storage: [9, 8, 9, 9],
        };

        // face down can't move even to a free column
        assert_that!(state.can_move_top_left_to_column(0, 7), eq(false));

        // dragon can move only to a free column
        assert_that!(state.can_move_top_left_to_column(1, 0), eq(false));
        assert_that!(state.can_move_top_left_to_column(1, 1), eq(false));
        assert_that!(state.can_move_top_left_to_column(1, 2), eq(false));
        assert_that!(state.can_move_top_left_to_column(1, 7), eq(true));

        // regular card can move onto another card of a different suit and
        // one lower value
        assert_that!(state.can_move_top_left_to_column(2, 0), eq(false));
        assert_that!(state.can_move_top_left_to_column(2, 1), eq(false));
        assert_that!(state.can_move_top_left_to_column(2, 2), eq(true));
        assert_that!(state.can_move_top_left_to_column(2, 7), eq(true));

        state.move_top_left_to_column(2, 2);
        assert_that!(state.top_left_storage.len(), eq(2));
        assert_that!(state.columns[2].len(), eq(2));
        assert_that!(
            state.columns[2].last().unwrap(),
            eq(&Card {
                suit: Red,
                value: Some(8)
            })
        );
    }

    #[test]
    fn test_can_move_column_to_top_left() {
        let mut state = GameState {
            columns: [
                vec![Card {
                    suit: Red,
                    value: Some(9),
                }],
                vec![Card {
                    suit: Red,
                    value: Some(8),
                }],
                vec![Card {
                    suit: Red,
                    value: Some(7),
                }],
                vec![
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                ],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
            ],
            top_right_storage: [9, 6, 9, 9],
        };

        // given a single empty slot in the top left
        assert_that!(state.top_left_storage.len(), eq(2));

        // Can't move if there is not any cards in the column
        assert_that!(state.can_move_column_to_top_left(7), eq(false));

        // Can move if there is any card in the column
        assert_that!(state.can_move_column_to_top_left(0), eq(true));
        assert_that!(state.can_move_column_to_top_left(1), eq(true));
        assert_that!(state.can_move_column_to_top_left(2), eq(true));
        assert_that!(state.can_move_column_to_top_left(3), eq(true));

        // Moving a card causes it to disappear from the column
        let moved_card = *state.columns[0].last().unwrap();
        state.move_column_to_top_left(0);
        assert_that!(state.columns[0].len(), eq(0));
        // The top left storage should be filled up
        assert_that!(state.top_left_storage.len(), eq(3));
        // The moved card should appear in the top left
        assert_that!(state.top_left_storage, contains(eq(moved_card)));

        // Now that the top left is filled up, no cards can be moved there
        assert_that!(state.can_move_column_to_top_left(1), eq(false))
    }

    #[test]
    fn test_collect_dragons() {
        let mut state = GameState {
            columns: [
                vec![Card {
                    suit: Green,
                    value: None,
                }],
                vec![Card {
                    suit: Green,
                    value: None,
                }],
                vec![Card {
                    suit: Green,
                    value: None,
                }],
                vec![Card {
                    suit: Green,
                    value: None,
                }],
                vec![
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: None,
                    },
                ],
                vec![Card {
                    suit: Black,
                    value: None,
                }],
                vec![
                    Card {
                        suit: Black,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: Some(9),
                    },
                ],
                vec![Card {
                    suit: Black,
                    value: None,
                }],
            ],
            top_left_storage: vec![Card {
                suit: Red,
                value: None,
            }],
            top_right_storage: [1, 8, 9, 9],
        };

        assert_that!(state.can_collect_dragons(Green), eq(true));
        assert_that!(state.can_collect_dragons(Red), eq(false));
        assert_that!(state.can_collect_dragons(Black), eq(false));

        state.collect_dragons(Green);
        assert_that!(state.columns[0].is_empty(), eq(true));
        assert_that!(state.columns[1].is_empty(), eq(true));
        assert_that!(state.columns[2].is_empty(), eq(true));
        assert_that!(state.columns[3].is_empty(), eq(true));
        assert_that!(
            state.top_left_storage.contains(&Card {
                suit: FaceDown,
                value: None
            }),
            eq(true)
        );
        assert_that!(
            state.top_left_storage.contains(&Card {
                suit: Red,
                value: None
            }),
            eq(true)
        );
        assert_that!(state.top_left_storage.len(), eq(2));

        state.collect_dragons(Red);
        assert_that!(state.columns[4].is_empty(), eq(true));
        assert_that!(state.top_left_storage.len(), eq(2));
    }

    #[test]
    fn test_can_not_collect_dragons_when_storage_full() {
        let mut state = GameState {
            columns: [
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![Card {
                    suit: Red,
                    value: None,
                }],
                vec![],
                vec![Card {
                    suit: Black,
                    value: None,
                }],
                vec![Card {
                    suit: Black,
                    value: None,
                }],
                vec![Card {
                    suit: Black,
                    value: None,
                }],
            ],
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: Red,
                    value: Some(9),
                },
                Card {
                    suit: Black,
                    value: None,
                },
            ],
            top_right_storage: [1, 8, 9, 9],
        };

        // We can't collect dragons if the top left storage is full
        assert_that!(state.can_collect_dragons(Red), eq(false));
        // Except if the blocking card is a dragon that we wanted to collect
        assert_that!(state.can_collect_dragons(Black), eq(true));

        state.collect_dragons(Black);
        assert_that!(state.top_left_storage.len(), eq(3));

        for column in &state.columns {
            assert_that!(
                column.contains(&Card {
                    suit: Black,
                    value: None
                }),
                eq(false)
            );
        }
    }

    #[test]
    fn test_move_column_to_other_column() {
        let state = GameState {
            columns: [
                vec![Card {
                    suit: Red,
                    value: Some(9),
                }],
                vec![
                    Card {
                        suit: Green,
                        value: Some(8),
                    },
                    Card {
                        suit: Black,
                        value: Some(7),
                    },
                ],
                vec![Card {
                    suit: Red,
                    value: Some(8),
                }],
                vec![
                    Card {
                        suit: Black,
                        value: Some(9),
                    },
                    Card {
                        suit: Green,
                        value: Some(9),
                    },
                    Card {
                        suit: Black,
                        value: Some(8),
                    },
                ],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: FaceDown,
                    value: None,
                },
            ],
            top_right_storage: [1, 7, 7, 6],
        };

        // Can't move if the source column is empty
        assert_that!(
            state.can_move_column_to_other_column(MoveColumnParameters {
                from_column_index: 4,
                to_column_index: 7,
                stack_size: 1,
            }),
            eq(false)
        );

        // Can't move if the source column stack size is less than the
        // requested stack size
        assert_that!(
            state.can_move_column_to_other_column(MoveColumnParameters {
                from_column_index: 0,
                to_column_index: 7,
                stack_size: 2,
            }),
            eq(false)
        );

        // Can move to an empty column
        assert_that!(
            state.can_move_column_to_other_column(MoveColumnParameters {
                from_column_index: 0,
                to_column_index: 7,
                stack_size: 1,
            }),
            eq(true)
        );
        let mut state_copy = state.clone();
        let card_to_move = *state_copy.columns[0].last().unwrap();
        state_copy.move_column_to_other_column(MoveColumnParameters {
            from_column_index: 0,
            to_column_index: 7,
            stack_size: 1,
        });
        assert_that!(state_copy.columns[0].is_empty(), eq(true));
        assert_that!(state_copy.columns[0].contains(&card_to_move), eq(false));
        assert_that!(state_copy.columns[7].len(), eq(1));
        assert_that!(state_copy.columns[7].contains(&card_to_move), eq(true));

        // Can move stack to empty column
        assert_that!(
            state.can_move_column_to_other_column(MoveColumnParameters {
                from_column_index: 3,
                to_column_index: 7,
                stack_size: 2,
            }),
            eq(true)
        );

        // Can move stack on top of another card
        assert_that!(
            state.can_move_column_to_other_column(MoveColumnParameters {
                from_column_index: 1,
                to_column_index: 0,
                stack_size: 2,
            }),
            eq(true)
        );
        let mut state_copy = state;
        let cards_to_move = &state_copy.columns[1][state_copy.columns[1].len() - 2..].to_vec();
        state_copy.move_column_to_other_column(MoveColumnParameters {
            from_column_index: 1,
            to_column_index: 0,
            stack_size: 2,
        });
        assert_that!(state_copy.columns[1].is_empty(), eq(true));
        for card_to_move in cards_to_move {
            assert_that!(state_copy.columns[1].contains(card_to_move), eq(false));
        }
        assert_that!(state_copy.columns[0].len(), eq(3));
        for card_to_move in cards_to_move {
            assert_that!(state_copy.columns[0].contains(card_to_move), eq(true));
        }
    }

    #[test]
    fn json_serialize() {
        let state = GameState {
            top_left_storage: vec![
                Card {
                    suit: FaceDown,
                    value: None,
                },
                Card {
                    suit: Red,
                    value: Some(5),
                },
            ],
            top_right_storage: [1, 2, 3, 4],
            columns: [
                vec![Card {
                    suit: Black,
                    value: None,
                }],
                vec![
                    Card {
                        suit: Red,
                        value: None,
                    },
                    Card {
                        suit: Red,
                        value: Some(9),
                    },
                ],
                vec![Card {
                    suit: Green,
                    value: Some(2),
                }],
                vec![Card {
                    suit: Green,
                    value: Some(1),
                }],
                vec![Card {
                    suit: Green,
                    value: Some(4),
                }],
                vec![Card {
                    suit: Green,
                    value: Some(5),
                }],
                vec![],
                vec![Card {
                    suit: Black,
                    value: Some(9),
                }],
            ],
        };

        let json = serde_json::to_string(&state).unwrap();
        let expected_json = "{\"top_left_storage\":[{\"suit\":\"FaceDown\",\"value\":null},{\"suit\":\"Red\",\"value\":5}],\"top_right_storage\":[1,2,3,4],\"columns\":[[{\"suit\":\"Black\",\"value\":null}],[{\"suit\":\"Red\",\"value\":null},{\"suit\":\"Red\",\"value\":9}],[{\"suit\":\"Green\",\"value\":2}],[{\"suit\":\"Green\",\"value\":1}],[{\"suit\":\"Green\",\"value\":4}],[{\"suit\":\"Green\",\"value\":5}],[],[{\"suit\":\"Black\",\"value\":9}]]}";
        println!("{}", json);

        assert_that!(json, eq(expected_json));
        let parsed: GameState = serde_json::from_str(json.as_str()).unwrap();
        assert_that!(parsed, eq(state));
    }
}
