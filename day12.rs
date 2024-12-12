use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let mut grid = Vec::new();

    for line in io::stdin().lock().lines() {
        grid.push(line.unwrap().chars().collect())
    }

    part1(&grid);
}

fn part1(grid: &[Vec<char>]) {
    fn visit(
        grid: &[Vec<char>],
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> (u64, u64) {
        if visited.contains(&(i, j)) {
            return (0, 0);
        }

        visited.insert((i, j));

        let DIRS: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let cur = grid[i as usize][j as usize];

        let mut neighbors = Vec::new();

        let mut res = (0, 0);

        for (idx, &(dx, dy)) in DIRS.iter().enumerate() {
            let (new_x, new_y) = (i as i64 + dx, j as i64 + dy);

            if new_x >= 0
                && new_y >= 0
                && new_x < grid.len() as i64
                && new_y < grid[0].len() as i64
                && grid[new_x as usize][new_y as usize] == cur
            {
                let (area, fence) = visit(grid, new_x as usize, new_y as usize, visited);
                res.1 += fence;
                res.0 += area;
                neighbors.push((new_x, new_y));
            }
        }

        if neighbors.len() == 0 {
            return (1 + res.0, res.1 + 4);
        } else if neighbors.len() == 1 {
            return (1 + res.0, res.1 + 3);
        } else if neighbors.len() == 2 {
            return (1 + res.0, res.1 + 2);
        } else if neighbors.len() == 3 {
            return (1 + res.0, res.1 + 1);
        } else if neighbors.len() == 4 {
            return (1 + res.0, res.1);
        } else {
            panic!("asdfasdf")
        }
    }

    let mut visited = HashSet::new();

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited.contains(&(i, j)) {
                let (a, b) = visit(&grid, i, j, &mut visited);

                println!("{} {} {} {}", i, j, a, b);

                sum += a * b;
            }
        }
    }

    // for (key, val) in area_map.iter() {
    //     println!("{} {}", key, val);
    //     // sum += val * fence_map[key];
    // }

    println!("{}", sum);
}
