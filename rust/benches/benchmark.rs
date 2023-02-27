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

    c.bench_function("benchmark", |b| {
        b.iter(|| {
            for state in &states {
                let mut game = Game::new();
                game.play(black_box(state.clone()));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
