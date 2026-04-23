// Name: Lyla Kiani
// Email: lyla.s.kiani@vanderbilt.edu
//
// Command-line entry point for Sudoku solver.

use std::io::{self, Write};

use cs3270_project3::{format_grid, read_puzzle_from_file, solve};

/// Runs command-line Sudoku solver workflow.
fn main() {
    println!("Enter puzzle file path (example: txt/sudoku-test1.txt).");
    print!("Press <Enter> to use txt/sudoku-test1.txt: ");
    if let Err(err) = io::stdout().flush() {
        eprintln!("Failed to flush output: {err}");
        return;
    }

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {err}");
        return;
    }

    let filename = if input.trim().is_empty() {
        "txt/sudoku-test1.txt"
    } else {
        input.trim()
    };

    let mut puzzle = match read_puzzle_from_file(filename) {
        Ok(grid) => grid,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    println!("\nInitial puzzle:");
    println!("{}", format_grid(&puzzle));

    if solve(&mut puzzle) {
        println!("Solved puzzle:");
        println!("{}", format_grid(&puzzle));
    } else {
        println!("This puzzle has no solution.");
    }
}
