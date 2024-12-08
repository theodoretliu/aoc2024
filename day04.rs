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

fn check_pattern(
    grid: &[Vec<char>],
    i: usize,
    j: usize,
    offsets: &[(i32, i32)],
    pattern: &[char],
) -> bool {
    pattern.iter().enumerate().all(|(k, &c)| {
        let (di, dj) = offsets[k];
        let new_i = (i as i32 + di) as usize;
        let new_j = (j as i32 + dj) as usize;
        grid.get(new_i)
            .and_then(|row| row.get(new_j))
            .map_or(false, |&cell| cell == c)
    })
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let patterns = [
        (
            vec!['X', 'M', 'A', 'S'],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        ), // Horizontal
        (
            vec!['S', 'A', 'M', 'X'],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        ),
        (
            vec!['X', 'M', 'A', 'S'],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        ), // Vertical
        (
            vec!['S', 'A', 'M', 'X'],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        ),
        (
            vec!['X', 'M', 'A', 'S'],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        ), // Diagonal
        (
            vec!['S', 'A', 'M', 'X'],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
        ),
        (
            vec!['X', 'M', 'A', 'S'],
            vec![(0, 3), (1, 2), (2, 1), (3, 0)],
        ), // Anti-diagonal
        (
            vec!['S', 'A', 'M', 'X'],
            vec![(0, 3), (1, 2), (2, 1), (3, 0)],
        ),
    ];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for (pattern, offsets) in &patterns {
                if check_pattern(grid, i, j, offsets, pattern) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for i in 0..grid.len().saturating_sub(2) {
        for j in 0..grid[i].len().saturating_sub(2) {
            if grid[i + 1][j + 1] == 'A' {
                let corners = [
                    grid[i][j],         // Top-left
                    grid[i][j + 2],     // Top-right
                    grid[i + 2][j],     // Bottom-left
                    grid[i + 2][j + 2], // Bottom-right
                ];

                let m_count = corners.iter().filter(|&&c| c == 'M').count();
                let s_count = corners.iter().filter(|&&c| c == 'S').count();

                if m_count == 2 && s_count == 2 && corners[0] != corners[3] {
                    count += 1;
                }
            }
        }
    }

    count
}
