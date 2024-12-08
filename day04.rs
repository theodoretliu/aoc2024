use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        if !chars.is_empty() {
            grid.push(chars);
        }
    }

    println!("{:?}", part1(&grid));
    println!("{:?}", part2(&grid));
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() - 3 {
            if grid[i][j] == 'X'
                && grid[i][j + 1] == 'M'
                && grid[i][j + 2] == 'A'
                && grid[i][j + 3] == 'S'
            {
                count += 1;
            } else if grid[i][j] == 'S'
                && grid[i][j + 1] == 'A'
                && grid[i][j + 2] == 'M'
                && grid[i][j + 3] == 'X'
            {
                count += 1;
            }
        }
    }

    for i in 0..grid.len() - 3 {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X'
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S'
            {
                count += 1;
            } else if grid[i][j] == 'S'
                && grid[i + 1][j] == 'A'
                && grid[i + 2][j] == 'M'
                && grid[i + 3][j] == 'X'
            {
                count += 1;
            }
        }
    }

    for i in 0..grid.len() - 3 {
        for j in 0..grid[i].len() - 3 {
            if grid[i][j] == 'X'
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S'
            {
                count += 1;
            } else if grid[i][j] == 'S'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j + 2] == 'M'
                && grid[i + 3][j + 3] == 'X'
            {
                count += 1;
            }
        }
    }

    for i in 0..grid.len() - 3 {
        for j in 0..grid[i].len() - 3 {
            if grid[i][j + 3] == 'X'
                && grid[i + 1][j + 2] == 'M'
                && grid[i + 2][j + 1] == 'A'
                && grid[i + 3][j] == 'S'
            {
                count += 1;
            } else if grid[i][j + 3] == 'S'
                && grid[i + 1][j + 2] == 'A'
                && grid[i + 2][j + 1] == 'M'
                && grid[i + 3][j] == 'X'
            {
                count += 1;
            }
        }
    }

    count
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for i in 0..grid.len() - 2 {
        for j in 0..grid[i].len() - 2 {
            if grid[i + 1][j + 1] == 'A' {
                // Check corners in clockwise order starting from top-left
                let corners = [
                    grid[i][j],
                    grid[i][j + 2], // Top corners
                    grid[i + 2][j],
                    grid[i + 2][j + 2], // Bottom corners
                ];

                // Check if we have 2 'M's and 2 'S's in corners
                let m_count = corners.iter().filter(|&&a| a == 'M').count();
                let s_count = corners.iter().filter(|&&a| a == 'S').count();

                if m_count == 2 && s_count == 2 {
                    // Check if the Ms and Ss are adjacent
                    if (corners[0] == corners[3]) {
                        continue; // Ms or Ss are on same side, not adjacent
                    }
                    count += 1;
                }
            }
        }
    }

    count
}
