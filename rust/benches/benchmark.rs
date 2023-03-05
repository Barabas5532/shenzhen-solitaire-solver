use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust::*;

#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn criterion_benchmark(c: &mut Criterion) {
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

    c.bench_function("benchmark", |b| {
        b.iter(|| {
            for state in &states {
                let mut game = Game::new(1);
                game.play(black_box(state.clone()));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
