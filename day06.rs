use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_pos = None;

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();

        if let Some(j) = chars.iter().position(|&c| c == '^') {
            start_pos = Some((i, j));
        }

        if !chars.is_empty() {
            grid.push(chars);
        }
    }

    let start_pos = start_pos.expect("No '^' found in grid");

    println!("{}", part1(&grid, start_pos));
    println!("{}", part2(&grid, start_pos));
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(grid: &[Vec<char>], start_pos: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut dir_idx = 0;
    let mut pos = start_pos;

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    visited.insert(pos);

    loop {
        let (di, dj) = DIRECTIONS[dir_idx];
        let new_pos = (pos.0 as i32 + di, pos.1 as i32 + dj);

        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= height || new_pos.1 >= width {
            break;
        }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            dir_idx = (dir_idx + 1) % DIRECTIONS.len();
        } else {
            pos = (new_pos.0 as usize, new_pos.1 as usize);
            visited.insert(pos);
        }
    }

    visited.len()
}

fn part2(grid: &[Vec<char>], start_pos: (usize, usize)) -> usize {
    fn evaluate_grid(grid: &[Vec<char>], start_pos: (usize, usize)) -> bool {
        let mut visited = HashSet::new();
        let mut dir_idx = 0;
        let mut pos = start_pos;

        visited.insert((pos, dir_idx));

        loop {
            let (di, dj) = DIRECTIONS[dir_idx];
            let new_pos = (pos.0 as i32 + di, pos.1 as i32 + dj);

            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid.len() as i32
                || new_pos.1 >= grid[0].len() as i32
            {
                return false;
            }

            if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                dir_idx = (dir_idx + 1) % DIRECTIONS.len();
            } else {
                pos = (new_pos.0 as usize, new_pos.1 as usize);

                if visited.contains(&(pos, dir_idx)) {
                    return true;
                }

                visited.insert((pos, dir_idx));
            }
        }
    }

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut copy: Vec<Vec<char>> = grid.iter().map(|row| row.to_vec()).collect();
            copy[i][j] = '#';

            if evaluate_grid(&copy, start_pos) {
                count += 1;
            }
        }
    }

    count
}
