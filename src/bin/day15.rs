use std::fs;
use std::io::{self, Read};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    let mut lines = input.lines();

    // Parse grid
    let mut grid = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect::<Vec<_>>());
    }

    let mut directions = Vec::new();

    // Parse directions
    while let Some(line) = lines.next() {
        directions.extend(
            line.chars()
                .map(|c| match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid direction character"),
                })
                .collect::<Vec<_>>(),
        );
    }
    println!("Grid size: {}x{}", grid[0].len(), grid.len());
    println!("Number of directions: {}", directions.len());

    part1(&grid, &directions);
}

fn part1(grid: &Vec<Vec<char>>, directions: &Vec<Direction>) {
    let mut grid = grid.clone();
    let mut robot_position = None;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                grid[i][j] = '.';
                robot_position = Some((i, j));
                break;
            }
        }
    }

    let mut robot_position = robot_position.unwrap();

    for direction in directions {
        let new_position = match direction {
            Direction::Up => (robot_position.0 - 1, robot_position.1),
            Direction::Down => (robot_position.0 + 1, robot_position.1),
            Direction::Left => (robot_position.0, robot_position.1 - 1),
            Direction::Right => (robot_position.0, robot_position.1 + 1),
        };

        match grid[new_position.0][new_position.1] {
            '#' => {}

            'O' => {
                let search_delta: (i64, i64) = match direction {
                    Direction::Up => (-1, 0),
                    Direction::Down => (1, 0),
                    Direction::Left => (0, -1),
                    Direction::Right => (0, 1),
                };

                let mut consideration = (new_position.0 as i64, new_position.1 as i64);

                let mut considered_position = Vec::new();

                while grid[consideration.0 as usize][consideration.1 as usize] != '#'
                    && grid[consideration.0 as usize][consideration.1 as usize] != '.'
                {
                    considered_position.push(consideration);

                    consideration = (
                        consideration.0 as i64 + search_delta.0,
                        consideration.1 as i64 + search_delta.1,
                    );
                }

                if grid[consideration.0 as usize][consideration.1 as usize] != '#' {
                    grid[consideration.0 as usize][consideration.1 as usize] = 'O';
                    grid[new_position.0][new_position.1] = '.';

                    robot_position = new_position;
                }
            }
            '.' => {
                robot_position = new_position;
            }

            _ => unreachable!(),
        }

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if (i, j) == robot_position {
                    print!("@");
                } else {
                    print!("{}", grid[i][j]);
                }
            }
            println!();
        }

        println!();
    }

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += 100 * i + j;
            }
        }
    }

    println!("{}", sum);
}
