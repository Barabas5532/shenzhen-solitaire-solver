use crate::card::*;

#[derive(Debug, Clone)]
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

    /*
    def can_move_top_left_to_top_right_storage(self, index: int) -> bool:
        if len(self.top_left_storage) <= index:
            return False

        card = self.top_left_storage[index]
        return (
            card.value is not None
            and self.top_right_storage[card.suit] == card.value - 1
        )

    def move_top_left_to_top_right_storage(self, index: int) -> None:
        card = self.top_left_storage.pop(index)
        self.top_right_storage[card.suit] = card.value
    */

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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Suit::{Black, FaceDown, Green, Red, Special};
    use googletest::matchers::*;
    use googletest::*;

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

            /*
            for s in result{
                println!(s)
            }
             */
        }
    }
}
