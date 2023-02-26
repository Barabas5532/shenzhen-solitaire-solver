use crate::card::Suit::Special;

use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Suit {
    Special,
    Red,
    Green,
    Black,
    FaceDown,
}

impl TryFrom<usize> for Suit {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Suit::Special as usize => Ok(Suit::Special),
            x if x == Suit::Red as usize => Ok(Suit::Red),
            x if x == Suit::Green as usize => Ok(Suit::Green),
            x if x == Suit::Black as usize => Ok(Suit::Black),
            x if x == Suit::FaceDown as usize => Ok(Suit::FaceDown),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Option<u8>,
}

impl Card {
    pub fn is_dragon(&self) -> bool {
        self.value == None
    }

    pub fn is_dragon_with_suit(&self, suit: Suit) -> bool {
        self == &Card { suit, value: None }
    }

    pub fn can_be_moved_on_top_of(&self, other: &Self) -> bool {
        if [self.suit, other.suit].contains(&Special) {
            return false;
        }

        assert!([Suit::Black, Suit::Green, Suit::Red].contains(&self.suit));

        // can't move on top of dragon
        if other.is_dragon() {
            return false;
        }

        // dragon can't go on top of any other card
        if self.is_dragon() {
            return false;
        }

        assert_ne!(other.value, None);

        return self.suit != other.suit && self.value.unwrap() == other.value.unwrap() - 1;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let colors = ["🟦", "🟥", "🟩", "⬛"];
        f.write_str(&match { self.suit } {
            Suit::FaceDown => String::from("xxx"),
            _ => format!(
                "{}{}",
                colors[self.suit as usize],
                match { self.value } {
                    None => String::from("x"),
                    Some(value) => format!("{}", value),
                }
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::matchers::{eq, not};
    use googletest::{assert_that, pointwise};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_hashable() {
        let a = Card {
            suit: Suit::Red,
            value: Some(1),
        };
        let b = Card {
            suit: Suit::Red,
            value: Some(1),
        };
        let c = Card {
            suit: Suit::Green,
            value: Some(1),
        };
        let d = Card {
            suit: Suit::Red,
            value: Some(2),
        };

        let a_hash = calculate_hash(&a);
        let b_hash = calculate_hash(&b);
        let c_hash = calculate_hash(&c);
        let d_hash = calculate_hash(&d);

        assert_that!(a, eq(b));
        assert_that!(a_hash, eq(b_hash));

        assert_that!(a, not(eq(c)));
        assert_that!(a_hash, not(eq(c_hash)));

        assert_that!(a, not(eq(d)));
        assert_that!(a_hash, not(eq(d_hash)));
    }

    #[test]
    fn test_sorting() {
        let test_data = [
            (
                [
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(4),
                    },
                ],
                [
                    Card {
                        suit: Suit::Red,
                        value: Some(4),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                ],
            ),
            (
                [
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                ],
                [
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                ],
            ),
            (
                [
                    Card {
                        suit: Suit::Black,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                ],
                [
                    Card {
                        suit: Suit::Red,
                        value: Some(5),
                    },
                    Card {
                        suit: Suit::Black,
                        value: Some(5),
                    },
                ],
            ),
            (
                [
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                ],
                [
                    Card {
                        suit: Suit::Red,
                        value: None,
                    },
                    Card {
                        suit: Suit::Black,
                        value: None,
                    },
                ],
            ),
        ];

        for mut test_case in test_data {
            test_case.0.sort();
            assert_that!(test_case.0, pointwise!(eq, test_case.1));
        }
    }
}
