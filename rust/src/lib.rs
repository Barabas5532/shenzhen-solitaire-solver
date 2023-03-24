use pyo3::prelude::*;

mod card;
mod game;
mod game_state;

pub use card::*;
pub use game::*;
pub use game_state::*;

/// Solves a game.
///
/// The argument and return value are JSON strings.
#[pyfunction]
fn solve_game(state: String) -> PyResult<String> {
    let state: GameState = serde_json::from_str(state.as_str()).unwrap();
    let mut game = Game::new();
    let solution = game.play(state);

    match serde_json::to_string(&solution) {
        Ok(solution) => Ok(solution),
        Err(_) => {
            panic!("Failed to serialise JSON")
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn shenzhen_solitaire_solver_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_game, m)?)?;
    Ok(())
}
