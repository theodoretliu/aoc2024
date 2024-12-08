use std::collections::HashMap;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

fn main() {
    let grid = parse_input();

    let char_positions = find_char_positions(&grid);

    let antinode_grid = find_antinodes(&grid, &char_positions);
    let count = count_antinodes(&antinode_grid);

    println!("{}", count);
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

fn find_antinodes(grid: &Grid, char_positions: &HashMap<char, Vec<Position>>) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antinode_grid = vec![vec![false; cols]; rows];

    for positions in char_positions.values() {
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

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let char_positions = find_char_positions(grid);
    // let antinode_grid = find_antinodes(grid, &char_positions); let count = count_antinodes(&antinode_grid);

    let mut antinodes = vec![vec![false; grid[0].len()]; grid.len()];

    for (_ch, positions) in char_positions {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (row1, col1) = positions[i];
                let (row2, col2) = positions[j];

                let dx = row2 as i64 - row1 as i64;
                let dy = col2 as i64 - col1 as i64;

                let mut cur_pos = (row1 as i64, col1 as i64);
                antinodes[cur_pos.0 as usize][cur_pos.1 as usize] = true;

                loop {
                    let next_pos = (cur_pos.0 as i64 + dx, cur_pos.1 as i64 + dy);
                    if is_valid_position(next_pos.0, grid.len())
                        && is_valid_position(next_pos.1, grid[0].len())
                    {
                        antinodes[next_pos.0 as usize][next_pos.1 as usize] = true;
                        cur_pos = next_pos;
                    } else {
                        break;
                    }
                }

                cur_pos = (row1 as i64, col1 as i64);
                loop {
                    let next_pos = (cur_pos.0 as i64 - dx, cur_pos.1 as i64 - dy);
                    if is_valid_position(next_pos.0, grid.len())
                        && is_valid_position(next_pos.1, grid[0].len())
                    {
                        antinodes[next_pos.0 as usize][next_pos.1 as usize] = true;
                        cur_pos = next_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let count = antinodes
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x)
        .count();

    count
}
