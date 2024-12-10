use std::io::{self, BufRead};

use std::collections::HashSet;
fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    // part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<u32>>) {
    let mut visited = HashSet::new();

    let mut sum = 0;

    fn f(
        grid: &Vec<Vec<u32>>,
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

        let mut directions = Vec::new();

        // Check up
        if i > 0 {
            directions.push((i - 1, j));
        }
        // Check down
        if i < grid.len() - 1 {
            directions.push((i + 1, j));
        }
        // Check left
        if j > 0 {
            directions.push((i, j - 1));
        }
        // Check right
        if j < grid[0].len() - 1 {
            directions.push((i, j + 1));
        }

        let mut reachable = HashSet::new();

        for (next_i, next_j) in directions {
            if grid[next_i][next_j] == grid[i][j] + 1 {
                reachable.extend(f(grid, next_i, next_j, visited));
            }
        }

        visited.remove(&(i, j));
        reachable
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                println!("{} {}", i, j);
                let res = f(grid, i, j, &mut visited);
                assert!(visited.len() == 0);
                println!("{}", res.len());

                sum += res.len();
            }
        }
    }

    println!("{}", sum);
}

fn part2(grid: &Vec<Vec<u32>>) {
    let mut visited = HashSet::new();

    let mut sum = 0;

    fn f(grid: &Vec<Vec<u32>>, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> u64 {
        if visited.contains(&(i, j)) {
            return 0;
        }

        if grid[i][j] == 9 {
            return 1;
        }

        visited.insert((i, j));

        let mut directions = Vec::new();

        // Check up
        if i > 0 {
            directions.push((i - 1, j));
        }
        // Check down
        if i < grid.len() - 1 {
            directions.push((i + 1, j));
        }
        // Check left
        if j > 0 {
            directions.push((i, j - 1));
        }
        // Check right
        if j < grid[0].len() - 1 {
            directions.push((i, j + 1));
        }

        // let mut reachable = HashSet::new();

        let mut sum = 0;

        for (next_i, next_j) in directions {
            if grid[next_i][next_j] == grid[i][j] + 1 {
                sum += f(grid, next_i, next_j, visited);
            }
        }

        visited.remove(&(i, j));
        sum
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                println!("{} {}", i, j);
                let res = f(grid, i, j, &mut visited);
                assert!(visited.len() == 0);
                println!("{}", res);

                sum += res;
            }
        }
    }

    println!("{}", sum);
}
