use std::collections::HashSet;

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut lines: Vec<HashSet<char>> = vec![HashSet::new(); board.len()];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); board.len()];
        let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); board.len() / 3 + (3 * (board.len() / 3))];

        for (i, line) in board.iter().enumerate() {
            for (j, val) in line.iter().enumerate() {
                if *val == '.' { continue; }
                if !lines[i].insert(*val) {
                    return false
                }
                if !cols[j].insert(*val) {
                    return false
                }
                let square_nb = (j / 3) + (3 * (i / 3));
                if !squares[square_nb].insert(*val) {
                    return false
                }
            }
        }

        true
}

fn main() {
    println!("Hello, world!");
}
