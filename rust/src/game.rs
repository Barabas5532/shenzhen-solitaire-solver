use crate::card::*;
use crate::game_state::*;
use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::iter::zip;

#[derive(Clone)]
pub enum GameMove {
    Start,
    ColumnToTopRightStorage {
        column: usize,
    },
    TopLeftToTopRightStorage {
        top_left_index: usize,
    },
    CollectDragons {
        suit: Suit,
    },
    ColumnToOtherColumn {
        from_column_index: usize,
        to_column_index: usize,
        stack_size: usize,
    },
    ToTopLeftStorage {
        column: usize,
    },
    TopLeftToColumn {
        top_left_index: usize,
        column_index: usize,
    },
}

struct PrioritisedGameState {
    priority: i32,
    state: GameState,
    path: Vec<GameState>,
    moves: Vec<GameMove>,
}

impl Eq for PrioritisedGameState {}
impl Ord for PrioritisedGameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for PrioritisedGameState {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl PartialOrd<Self> for PrioritisedGameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

pub struct Game {
    open: BinaryHeap<PrioritisedGameState>,
    closed: FxHashSet<GameState>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            open: BinaryHeap::new(),
            closed: FxHashSet::default(),
        }
    }

    pub fn play(&mut self, state: GameState) -> Option<Vec<(GameState, GameMove)>> {
        self.initialise(state);

        while !self.open.is_empty() {
            let head = self.open.pop().unwrap();
            let solution = self.expand_node(head);
            if solution.is_some() {
                return solution;
            }
        }

        None
    }

    fn heuristic(state: &GameState) -> i32 {
        // comments are indicating what result we get if the code below the
        // comment is removed
        let mut score = 0;

        // solution 0 length 173
        // solution 1 length 120
        // solution 2 length 275

        // Collected cards are good
        for value in state.top_right_storage {
            score += value as i32;
        }

        // solution 0 length 64
        // solution 1 length 76
        // solution 2 length 87

        // Cards hidden by dragons are bad
        let blocked_card_count = state
            .columns
            .iter()
            .filter(|column| column.iter().any(|card| card.is_dragon()))
            .fold(0i32, |acc, column| {
                acc + i32::try_from(column.len()).expect(
                    "Column lengths are bounded by the number of cards in \
                    play, which is always small",
                )
            });
        score -= blocked_card_count;

        score
    }

    fn initialise(&mut self, state: GameState) {
        assert!(self.open.is_empty());
        assert!(self.closed.is_empty());

        self.closed.insert(state.clone());
        let new_entry = PrioritisedGameState {
            priority: Self::heuristic(&state),
            path: vec![state.clone()],
            // NOTE: slight difference from python here. There is no zip longest
            // in rust, so instead we initialise this vector with a default
            // value. This makes it the same length as path, so that regular zip
            // works.
            moves: vec![GameMove::Start],
            state,
        };
        self.open.push(new_entry)
    }

    fn visit_node(&mut self, parent: &PrioritisedGameState, state: GameState, game_move: GameMove) {
        if !self.closed.contains(&state) {
            self.closed.insert(state.clone());
            // TODO this might work more efficiently using immutable collections
            // from the im crate
            //
            // Even better, we could store a reference to the previous state and
            // just iterate the resulting linked list to find the path
            let new_entry = PrioritisedGameState {
                priority: Self::heuristic(&state),
                path: parent.path.iter().chain([&state]).cloned().collect(),
                moves: parent.moves.iter().chain([&game_move]).cloned().collect(),
                state,
            };
            self.open.push(new_entry);
        }
    }

    fn expand_node(&mut self, state: PrioritisedGameState) -> Option<Vec<(GameState, GameMove)>> {
        // TODO we should make this more efficient by using a greedy algorithm
        //
        // That modification would expand a child node immediately if it has a
        // lower score than the current node. This version expands all children
        // before picking the next node to work on.
        //
        // Implementing this would require storing the state of all the loop
        // counters along with the game state, so that we can pick up where we
        // left off.
        //
        // Could a generator function yielding the next move help here? That
        // would keep its state for the next call.

        // the game forces us to move any cards to the top right storage if it's
        // a valid move
        //
        // This is also our base case, as the game is won when all cards have
        // been stacked here.
        //
        // The dragons might not all be collected yet, but that's fine, since if
        // we have all the numbered cards, the dragons are always free to
        // collect as the final move. This might also be automatic.
        //
        // TODO Does this also happen for cards in the top left storage?
        //      In any case, they would be collected to the top right as soon as
        //      they are moved away from the top left, there is no point keeping
        //      them there.
        //

        // TODO could move this after we make a move to spot the win 1 iteration
        //      sooner

        if state.state.is_solved() {
            return Some(zip(state.path, state.moves).clone().collect());
        }

        // Use a copy so we can reset the state after each move
        let mut state_copy = state.state.clone();

        for i in 0..8 {
            if state_copy.can_move_column_to_top_right_storage(i) {
                state_copy.move_column_to_top_right_storage(i);
                self.visit_node(
                    &state,
                    state_copy,
                    GameMove::ColumnToTopRightStorage { column: i },
                );

                // We have to make this move, the game won't let us do anything
                // else. If it results in a losing game, then we need to
                // backtrack
                //
                // TODO there might be some edge cases where the game doesn't
                // force you to make this move. Those could be the states where
                // this move actually makes you lose the game.
                //
                // E.G. when moving the card would mean a free columns stays
                // blocked by another card.
                return None;
            }
        }

        for i in 0..3 {
            if state_copy.can_move_top_left_to_top_right_storage(i) {
                state_copy.move_top_left_to_top_right_storage(i);
                self.visit_node(
                    &state,
                    state_copy,
                    GameMove::TopLeftToTopRightStorage { top_left_index: i },
                );

                // See comment above
                return None;
            }
        }

        // test out every possible move. The list of all moves are:
        // collect dragons
        for suit in [Suit::Red, Suit::Green, Suit::Black] {
            if state_copy.can_collect_dragons(suit) {
                state_copy.collect_dragons(suit);
                self.visit_node(&state, state_copy, GameMove::CollectDragons { suit });

                state_copy = state.state.clone();
            }
        }

        // move any set of cards from any column to any other column
        for from_column_index in 0..8 {
            for to_column_index in 0..8 {
                for stack_size in (1..9 + 1).rev() {
                    if state_copy.can_move_column_to_other_column(MoveColumnParameters {
                        from_column_index,
                        to_column_index,
                        stack_size,
                    }) {
                        state_copy.move_column_to_other_column(MoveColumnParameters {
                            from_column_index,
                            to_column_index,
                            stack_size,
                        });
                        self.visit_node(
                            &state,
                            state_copy,
                            GameMove::ColumnToOtherColumn {
                                from_column_index,
                                to_column_index,
                                stack_size,
                            },
                        );
                        state_copy = state.state.clone();
                    }
                }
            }
        }

        // move a card from the centre to the storage area
        for column_index in 0..8 {
            if state_copy.can_move_column_to_top_left(column_index) {
                state_copy.move_column_to_top_left(column_index);
                self.visit_node(
                    &state,
                    state_copy,
                    GameMove::ToTopLeftStorage {
                        column: column_index,
                    },
                );
                state_copy = state.state.clone();
            }
        }

        // move a card out of the top left storage area to a column
        for top_left_index in 0..3 {
            for column_index in 0..8 {
                if state_copy.can_move_top_left_to_column(top_left_index, column_index) {
                    state_copy.move_top_left_to_column(top_left_index, column_index);
                    self.visit_node(
                        &state,
                        state_copy,
                        GameMove::TopLeftToColumn {
                            top_left_index,
                            column_index,
                        },
                    );
                    state_copy = state.state.clone()
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::card::Suit::{FaceDown, Red};
    use googletest::assert_that;
    use googletest::matchers::*;

    fn solved() -> GameState {
        GameState {
            columns: [
                vec![],
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
                    value: None
                };
                3
            ],
            top_right_storage: [1, 9, 9, 9],
        }
    }

    fn almost_solved() -> GameState {
        GameState {
            columns: [
                vec![Card {
                    suit: Red,
                    value: Some(9),
                }],
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
                    value: None
                };
                3
            ],
            top_right_storage: [1, 8, 9, 9],
        }
    }

    #[test]
    fn test_is_solved() {
        let solved = solved();
        assert_that!(solved.is_solved(), eq(true));

        // The base case
        let mut game = Game::new();
        let result = game.play(solved.clone());
        assert_that!(result.is_none(), eq(false));
        let result = result.unwrap();
        assert_that!(result.len(), eq(1));
        assert_that!(&result[0].0, eq(&solved));
    }

    #[test]
    fn test_move_to_top_right_solve() {
        let almost_solved = almost_solved();
        assert_that!(almost_solved.is_solved(), eq(false));

        // Solved after a single iteration
        let mut game = Game::new();
        let result = game.play(almost_solved.clone());
        assert_that!(result.is_none(), eq(false));
        let result = result.unwrap();
        assert_that!(result.len(), eq(2));
        assert_that!(&result[0].0, eq(&almost_solved));
        assert_that!(result.last().unwrap().0.is_solved(), eq(true));
    }
}
