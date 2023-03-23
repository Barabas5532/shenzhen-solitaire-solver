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
fn solve_game(game: String) -> PyResult<String> {
    Ok("hello".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn shenzhen_solitaire_solver_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_game, m)?)?;
    Ok(())
}
