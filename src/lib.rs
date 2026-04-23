// Name: [Your Name]
// Email: [your.email@example.com]
//
// Core Sudoku implementation and utilities for Project 3.

pub mod puzzle_io;
pub mod solver;
pub mod view;

pub use puzzle_io::read_puzzle_from_file;
pub use solver::solve;
pub use view::format_grid;
