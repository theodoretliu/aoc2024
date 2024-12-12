use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    part1(&grid);
}

fn part1(grid: &[Vec<char>]) {
    fn visit(
        grid: &[Vec<char>],
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> (u64, u64) {
        if visited.contains(&pos) {
            return (0, 0);
        }

        visited.insert(pos);

        const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let (i, j) = pos;
        let current_char = grid[i][j];

        let mut neighbors = Vec::new();
        let mut result = (0, 0);

        for (dx, dy) in DIRECTIONS {
            let new_x = i as i64 + dx;
            let new_y = j as i64 + dy;

            if is_valid_position(new_x, new_y, grid)
                && grid[new_x as usize][new_y as usize] == current_char
            {
                let new_pos = (new_x as usize, new_y as usize);
                let (area, fence) = visit(grid, new_pos, visited);
                result.0 += area;
                result.1 += fence;
                neighbors.push((new_x, new_y));
            }
        }

        // Calculate area and fence based on number of neighbors
        let (additional_area, additional_fence) = match neighbors.len() {
            0 => (1, 4),
            1 => (1, 3),
            2 => (1, 2),
            3 => (1, 1),
            4 => (1, 0),
            _ => unreachable!("Invalid number of neighbors"),
        };

        (result.0 + additional_area, result.1 + additional_fence)
    }

    fn is_valid_position(x: i64, y: i64, grid: &[Vec<char>]) -> bool {
        x >= 0 && y >= 0 && x < grid.len() as i64 && y < grid[0].len() as i64
    }

    let mut visited = HashSet::new();
    let mut total_sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let pos = (i, j);
            if !visited.contains(&pos) {
                let (area, fence) = visit(grid, pos, &mut visited);
                total_sum += area * fence;
            }
        }
    }

    println!("{}", total_sum);
}
