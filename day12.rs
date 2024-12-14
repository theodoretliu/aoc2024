use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    part1(&grid);
    part2(&grid);
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

fn part2(grid: &[Vec<char>]) {
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

        let mut direction_pairs = Vec::new();
        for i in 0..DIRECTIONS.len() {
            direction_pairs.push((DIRECTIONS[i], DIRECTIONS[(i + 1) % DIRECTIONS.len()]));
        }

        for dir in DIRECTIONS {
            let pls1 = (i as i64 + dir.0, j as i64 + dir.1);
        }

        let mut tot_area = 1;

        let mut tot_internal_angle = 0;

        let mut num_adjacent_neighbor_pairs = 0;

        let mut num_concave_points = 0;

        let mut num_corners = 0;

        for (d1, d2) in direction_pairs {
            let pos1 = (i as i64 + d1.0, j as i64 + d1.1);
            let pos2 = (i as i64 + d2.0, j as i64 + d2.1);

            // check that both neighbors are in the grid and have the same character
            if is_valid_position(pos1.0, pos1.1, grid)
                && is_valid_position(pos2.0, pos2.1, grid)
                && grid[pos1.0 as usize][pos1.1 as usize] == current_char
                && grid[pos2.0 as usize][pos2.1 as usize] == current_char
            {
                num_adjacent_neighbor_pairs += 1;
                let corner_dir = (
                    if d1.0 == 0 { d2.0 } else { d1.0 },
                    if d1.1 == 0 { d2.1 } else { d1.1 },
                );
                let corner_pos = (i as i64 + corner_dir.0, j as i64 + corner_dir.1);

                // println!("{:?} {:?}", corner_dir, corner_pos);
                assert!(is_valid_position(corner_pos.0, corner_pos.1, grid));

                if grid[corner_pos.0 as usize][corner_pos.1 as usize] != current_char {
                    // println!("{:?} {:?}", corner_pos, corner_dir);
                    num_corners += 1;
                    num_concave_points += 1;
                }
            }
        }

        let mut num_neighbors = 0;
        for (dx, dy) in DIRECTIONS {
            let new_x = i as i64 + dx;
            let new_y = j as i64 + dy;

            if is_valid_position(new_x, new_y, grid)
                && grid[new_x as usize][new_y as usize] == current_char
            {
                num_neighbors += 1;

                let (inc_tot_area, inc_num_corners) =
                    visit(grid, (new_x as usize, new_y as usize), visited);
                tot_area += inc_tot_area;
                num_corners += inc_num_corners;
                // tot_internal_angle += _tot_internal_angle;
            }
        }

        num_corners += match num_neighbors {
            0 => 4,
            1 => 2,
            2 => {
                if num_adjacent_neighbor_pairs == 1 {
                    1
                } else if num_adjacent_neighbor_pairs == 0 {
                    0
                } else {
                    unreachable!("Invalid number of neighbors");
                }
            }
            3 => 0,
            4 => 0,
            _ => unreachable!("Invalid number of neighbors"),
        };

        return (tot_area, num_corners);
    }

    let mut visited = HashSet::new();
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let pos = (i, j);
            if !visited.contains(&pos) {
                let (area, num_corners) = visit(grid, pos, &mut visited);
                sum += area * num_corners;
            }
        }
    }

    println!("{}", sum);
}

fn is_valid_position(x: i64, y: i64, grid: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i64 && y < grid[0].len() as i64
}
