use std::collections::HashSet;
use std::io::{self, BufRead};

// Common direction checking logic used by both parts
fn get_valid_directions(grid: &[Vec<u32>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let deltas = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut directions = Vec::new();

    for (di, dj) in deltas {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 {
            directions.push((new_i as usize, new_j as usize));
        }
    }

    directions
}

fn main() {
    let stdin = io::stdin();
    let mut grid = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        grid.push(row);
    }

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &[Vec<u32>]) {
    let mut visited = HashSet::new();
    let mut sum = 0;

    // Returns set of reachable points from (i,j) that form a path of increasing digits
    fn traverse_path(
        grid: &[Vec<u32>],
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        if visited.contains(&(i, j)) {
            return HashSet::new();
        }

        if grid[i][j] == 9 {
            return HashSet::from([(i, j)]);
        }

        visited.insert((i, j));
        let mut reachable = HashSet::new();

        for (next_i, next_j) in get_valid_directions(grid, i, j) {
            if grid[next_i][next_j] == grid[i][j] + 1 {
                reachable.extend(traverse_path(grid, next_i, next_j, visited));
            }
        }

        visited.remove(&(i, j));
        reachable
    }

    // Find all paths starting from 0s
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                let path = traverse_path(grid, i, j, &mut visited);
                sum += path.len();
            }
        }
    }

    println!("{}", sum);
}

fn part2(grid: &[Vec<u32>]) {
    let mut visited = HashSet::new();
    let mut sum = 0;

    // Returns count of reachable points from (i,j) that form a path of increasing digits
    fn count_path(
        grid: &[Vec<u32>],
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> u64 {
        if visited.contains(&(i, j)) {
            return 0;
        }

        if grid[i][j] == 9 {
            return 1;
        }

        visited.insert((i, j));
        let mut count = 0;

        for (next_i, next_j) in get_valid_directions(grid, i, j) {
            if grid[next_i][next_j] == grid[i][j] + 1 {
                count += count_path(grid, next_i, next_j, visited);
            }
        }

        visited.remove(&(i, j));
        count
    }

    // Find all paths starting from 0s
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                let path_count = count_path(grid, i, j, &mut visited);
                sum += path_count;
            }
        }
    }

    println!("{}", sum);
}
