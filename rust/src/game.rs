use crate::card::Suit::FaceDown;
use crate::card::*;
use std::fmt::{write, Formatter};
use std::{cmp, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    // scratch pad to temporarily store cards
    // a space is lost when dragons are stacked here, represented by a
    // Card(Suit.FACE_DOWN, None)
    top_left_storage: Vec<Card>,

    // The aim of the game is to get all the cards stacked here
    top_right_storage: [u8; 4],

    // The main play area, where all of the cards are placed at the start
    columns: [Vec<Card>; 8],
}

impl GameState {
    // All the columns in the centre have no cards
    pub fn is_solved(&self) -> bool {
        for column in &self.columns {
            if column.len() != 0 {
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
        if self.columns[column_index].len() == 0 {
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

    fn can_move_top_left_to_top_right_storage(&self, top_left_index: usize) -> bool {
        if self.top_left_storage.len() <= top_left_index {
            return false;
        }

        let card = &self.top_left_storage[top_left_index];

        match { card.value } {
            None => false,
            Some(value) => self.top_right_storage[card.suit as usize] == value - 1,
        }
    }

    fn move_top_left_to_top_right_storage(&mut self, top_left_index: usize) {
        let card = self.top_left_storage.remove(top_left_index);
        self.top_right_storage[card.suit as usize] = card
            .value
            .expect("must call can_move_to_top_right_storage first");
    }

    fn can_move_top_left_to_column(&self, top_left_index: usize, column_index: usize) -> bool {
        if top_left_index >= self.top_left_storage.len() {
            return false;
        }

        let card_to_move = &self.top_left_storage[top_left_index];
        // Can't move collected dragons
        if card_to_move.suit == FaceDown {
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

    fn move_top_left_to_column(&mut self, top_left_index: usize, column_index: usize) {
        self.columns[column_index].push(self.top_left_storage.remove(top_left_index))
    }

    fn can_move_column_to_top_left(&self, column_index: usize) -> bool {
        !self.columns[column_index].is_empty() && self.top_left_storage.len() < 3
    }

    fn move_column_to_top_left(&mut self, column_index: usize) {
        self.top_left_storage
            .push(self.columns[column_index].pop().unwrap());
        assert!(self.top_left_storage.len() <= 3);
    }
}

/*
   def _tuple(self) -> tuple:
       tuple_column = [tuple(column) for column in self.columns]

       # columns are sorted by the bottom card to try to prevent useless
       # moves moving stacks to another empty column
       tuple_column = sorted(
           tuple_column,
           key=lambda x: x[0] if len(x) != 0 else Card(Suit.SPECIAL, None),
       )

       return (
           tuple(sorted(self.top_left_storage)),
           tuple(tuple_column),
           tuple(self.top_right_storage),
       )
*/

/*
impl PartialEq<Self> for GameState {
    fn eq(&self, other: &Self) -> bool {
        // columns are sorted by the bottom card to try to prevent useless
        // moves moving stacks to another empty column

        let get_bottom_card = |col: &Vec<Card>| {
            col.first().unwrap_or(&Card {
                suit: Suit::Special,
                value: None,
            })
        };
        let bottom_card_sort =
            |a: &Vec<Card>, b: &Vec<Card>| get_bottom_card(a).cmp(get_bottom_card(b));
        let columns = self.columns.sort_by(bottom_card_sort);
        let other_columns = other.columns.sort_by(bottom_card_sort);

        columns == other_columns
    }
}
 */

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

        let mut b = empty_columns.clone();
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

        // TODO Permutations of columns should not effect equality and hash
        // assert_that!(state_a, eq(state_b.clone()));
        // assert_that!(hash_a, eq(hash_b));

        assert_that!(state_b, eq(state_c));
        assert_that!(hash_b, eq(hash_c));

        assert_that!(state_a, not(eq(state_d)));
        assert_that!(hash_a, not(eq(hash_d)));
    }

    /*

       def test_hash_ignores_top_left_permutions(self) -> None:
           """We should be able to optimize the exution time by detecting
           more identical cycles where the only difference is the permuation
           of the cards in the top left corner

           E.G. a position with the top left storage having

           🟥x ⬛x 🟩3

           or

           🟩3 🟥x ⬛x

           is identical from a gameplay perspective
           """
           empty_columns: Columns = ([], [], [], [], [], [], [], [])

           a = GameState(
               empty_columns,
               [
                   Card(Suit.RED, None),
                   Card(Suit.BLACK, None),
                   Card(Suit.GREEN, 3),
               ],
           )

           b = GameState(
               empty_columns,
               [
                   Card(Suit.GREEN, 3),
                   Card(Suit.RED, None),
                   Card(Suit.BLACK, None),
               ],
           )

           self.assertEqual(hash(a), hash(b))

    */

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

    /*

        def test_can_move_column_to_top_left(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.RED, 9),
                    ],
                    [
                        Card(Suit.RED, 8),
                    ],
                    [
                        Card(Suit.RED, 7),
                    ],
                    [
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                    ],
                    [],
                    [],
                    [],
                    [],
                ),
                [
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.FACE_DOWN, None),
                ],
                [9, 6, 9, 9],
            )

            # Given a single empty slot in the top left
            self.assertEqual(2, len(state.top_left_storage))

            # Can't move if there is not any cards in the column
            self.assertFalse(state.can_move_column_to_top_left(7))

            # Can move if there is any card in the column
            self.assertTrue(state.can_move_column_to_top_left(0))
            self.assertTrue(state.can_move_column_to_top_left(1))
            self.assertTrue(state.can_move_column_to_top_left(2))
            self.assertTrue(state.can_move_column_to_top_left(3))

            # Moving a card causes it to disappear from the column
            moved_card = state.columns[0][-1]
            state.move_column_to_top_left(0)
            self.assertEqual(0, len(state.columns[0]))
            # The top left storage should be filled up
            self.assertEqual(3, len(state.top_left_storage))
            # The moved card should appear in the top left
            self.assertIn(moved_card, state.top_left_storage)

            # Now that the top left is filled up, no cards can be moved there
            self.assertFalse(state.can_move_column_to_top_left(1))

    */
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
        let moved_card = state.columns[0].last().unwrap().clone();
        state.move_column_to_top_left(0);
        assert_that!(state.columns[0].len(), eq(0));
        // The top left storage should be filled up
        assert_that!(state.top_left_storage.len(), eq(3));
        // The moved card should appear in the top left
        assert_that!(state.top_left_storage, contains(eq(moved_card)));

        // Now that the top left is filled up, no cards can be moved there
        assert_that!(state.can_move_column_to_top_left(1), eq(false))
    }
}
