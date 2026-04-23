// Name: Lyla Kiani
// Email: lyla.s.kiani@vanderbilt.edu
//
// Rendering Sudoku puzzles for terminal output.

use crate::solver::Grid;

/**
 * Returns grid formatted with the borders included
 *
 * @param grid - sudoku grid.
 * @return -grid with borders
 */
pub fn format_grid(grid: &Grid) -> String {
    let mut output = String::new();
    let border = "+-------+-------+-------+\n";

    for (row_idx, row) in grid.iter().enumerate() {
        if row_idx % 3 == 0 {
            output.push_str(border);
        }

        output.push('|');
        for (col_idx, value) in row.iter().enumerate() {
            if *value == 0 {
                output.push_str(" .");
            } else {
                output.push_str(&format!(" {value}"));
            }
            if (col_idx + 1) % 3 == 0 {
                output.push_str(" |");
            }
        }
        output.push('\n');
    }

    output.push_str(border);
    output
}
