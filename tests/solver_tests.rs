// Name: Lyla Kiani
// Email: lyla.s.kiani@vanderbilt.edu
//
// contains tests for sudoku solver

use cs3270_project3::{read_puzzle_from_file, solve};

#[test]
fn solves_sudoku_test1() {
    let mut puzzle = read_puzzle_from_file("txt/sudoku-test1.txt").expect("load test1");
    let expected = read_puzzle_from_file("txt/sudoku-test1-solution.txt").expect("load test1 solution");
    let solved = solve(&mut puzzle);

    assert!(solved, "Expected sudoku-test1 to be solvable.");
    assert_eq!(puzzle, expected);
}

#[test]
fn solves_sudoku_test2() {
    let mut puzzle = read_puzzle_from_file("txt/sudoku-test2.txt").expect("load test2");
    let expected = read_puzzle_from_file("txt/sudoku-test2-solution.txt").expect("load test2 solution");
    let solved = solve(&mut puzzle);

    assert!(solved, "Expected sudoku-test2 to be solvable.");
    assert_eq!(puzzle, expected);
}

#[test]
fn reports_unsolvable_puzzle() {
    let mut puzzle = read_puzzle_from_file("txt/sudoku-impossible.txt").expect("load impossible");
    let solved = solve(&mut puzzle);
    assert!(!solved, "Expected sudoku-impossible to be unsolvable.");
}
