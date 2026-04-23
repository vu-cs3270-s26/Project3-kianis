// Name: lyla kiani
// Email: lyla.s.kiani@vanderbilt.edu
//
// Backtracking sudoku solver

/// initialize grid type
pub type Grid = [[u8; 9]; 9];

/// main sudoku solver function
pub fn solve(grid: &mut Grid) -> bool {
    match find_empty(grid) {
        None => true,
        Some((row, col)) => {
            for value in 1..=9 {
                if can_place(grid, row, col, value) {
                    grid[row][col] = value;
                    if solve(grid) {
                        return true;
                    }
                    grid[row][col] = 0;
                }
            }
            false
        }
    }
}

/// Returns empty cell in grid
fn find_empty(grid: &Grid) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value == 0 {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

/// returns true if we can place value in grid
fn can_place(grid: &Grid, row: usize, col: usize, value: u8) -> bool {
    !in_row(grid, row, value)
        && !in_col(grid, col, value)
        && !in_subgrid(grid, row, col, value)
}

/// Returns true if value is already in row
fn in_row(grid: &Grid, row: usize, value: u8) -> bool {
    grid[row].contains(&value)
}

/// Returns true when value already exists in the column.
fn in_col(grid: &Grid, col: usize, value: u8) -> bool {
    (0..9).any(|row| grid[row][col] == value)
}

/// Returns true when value already exists in the 3x3 subgrid.
fn in_subgrid(grid: &Grid, row: usize, col: usize, value: u8) -> bool {
    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for r in start_row..start_row + 3 {
        for c in start_col..start_col + 3 {
            if grid[r][c] == value {
                return true;
            }
        }
    }
    false
}
