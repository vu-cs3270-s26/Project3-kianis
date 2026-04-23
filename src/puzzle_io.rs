// Name: lyla kiani
// Email: lyla.s.kiani@vanderbilt.edu
//
// read in sudoku files

use std::fs;
use std::path::Path;

use crate::solver::Grid;

/**
 * Read puzzle from file into sudoku solver.
 *
 * @param path Path to a puzzle text file.
 * @return Parsed 9x9 Sudoku grid or an error message.
 */
pub fn read_puzzle_from_file(path: &str) -> Result<Grid, String> {
    let text = fs::read_to_string(Path::new(path))
        .map_err(|err| format!("Failed to read '{path}': {err}"))?;

    parse_grid(&text)
}

/**
 * Reads grid contents into desired format for sudoku solver
 *
 * @param contents - raw puzzle text.
 * @return - written 9x9 sudoku grid or an error message.
 */
fn parse_grid(contents: &str) -> Result<Grid, String> {
    let values: Vec<u8> = contents
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|digit| digit as u8))
        .collect();

    if values.len() != 81 {
        return Err(format!(
            "Expected 81 values, but found {} values.",
            values.len()
        ));
    }

    let mut grid = [[0_u8; 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            grid[row][col] = values[row * 9 + col];
        }
    }

    Ok(grid)
}
