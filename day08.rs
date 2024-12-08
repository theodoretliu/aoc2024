use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

fn main() {
    let grid = parse_input();
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

fn find_char_positions(grid: &Grid) -> HashMap<char, Vec<Position>> {
    let mut char_positions = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch != '.' {
                char_positions
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((row_idx, col_idx));
            }
        }
    }

    char_positions
}

fn is_valid_position(pos: i64, max: usize) -> bool {
    pos >= 0 && pos < max as i64
}

fn find_antinodes_single(grid: &Grid, positions: &[Position]) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antinode_grid = vec![vec![false; cols]; rows];

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (row1, col1) = positions[i];
            let (row2, col2) = positions[j];

            let dx = row2 as i64 - row1 as i64;
            let dy = col2 as i64 - col1 as i64;

            // Check forward antinode
            let next_row = row2 as i64 + dx;
            let next_col = col2 as i64 + dy;
            if is_valid_position(next_row, rows) && is_valid_position(next_col, cols) {
                antinode_grid[next_row as usize][next_col as usize] = true;
            }

            // Check backward antinode
            let prev_row = row1 as i64 - dx;
            let prev_col = col1 as i64 - dy;
            if is_valid_position(prev_row, rows) && is_valid_position(prev_col, cols) {
                antinode_grid[prev_row as usize][prev_col as usize] = true;
            }
        }
    }

    antinode_grid
}

fn find_antinodes_line(grid: &Grid, positions: &[Position]) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antinode_grid = vec![vec![false; cols]; rows];

    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (row1, col1) = positions[i];
            let (row2, col2) = positions[j];

            let dx = row2 as i64 - row1 as i64;
            let dy = col2 as i64 - col1 as i64;

            // Mark all points in both directions
            for direction in [-1, 1] {
                let mut cur_pos = (row1 as i64, col1 as i64);
                loop {
                    if is_valid_position(cur_pos.0, rows) && is_valid_position(cur_pos.1, cols) {
                        antinode_grid[cur_pos.0 as usize][cur_pos.1 as usize] = true;
                        cur_pos.0 += dx * direction;
                        cur_pos.1 += dy * direction;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinode_grid
}

fn count_antinodes(antinode_grid: &[Vec<bool>]) -> usize {
    antinode_grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x)
        .count()
}

fn parse_input() -> Grid {
    let stdin = io::stdin();
    let mut grid = Vec::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    grid
}

fn part1(grid: &Grid) -> usize {
    let char_positions = find_char_positions(grid);
    let mut total_antinodes = vec![vec![false; grid[0].len()]; grid.len()];

    for positions in char_positions.values() {
        let antinodes = find_antinodes_single(grid, positions);
        for (row_idx, row) in antinodes.iter().enumerate() {
            for (col_idx, &val) in row.iter().enumerate() {
                total_antinodes[row_idx][col_idx] |= val;
            }
        }
    }

    count_antinodes(&total_antinodes)
}

fn part2(grid: &Grid) -> usize {
    let char_positions = find_char_positions(grid);
    let mut total_antinodes = vec![vec![false; grid[0].len()]; grid.len()];

    for positions in char_positions.values() {
        let antinodes = find_antinodes_line(grid, positions);
        for (row_idx, row) in antinodes.iter().enumerate() {
            for (col_idx, &val) in row.iter().enumerate() {
                total_antinodes[row_idx][col_idx] |= val;
            }
        }
    }

    count_antinodes(&total_antinodes)
}
