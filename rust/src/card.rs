use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Suit {
    Special,
    Red,
    Green,
    Black,
    FaceDown,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    pub suit: Suit,
    pub value: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::matchers::{eq, not};
    use googletest::{assert_that, pointwise};

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
