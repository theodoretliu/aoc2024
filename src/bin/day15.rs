use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

fn parse_input(input: &str) -> (Grid, Vec<Direction>) {
    let mut lines = input.lines();

    // Parse grid
    let mut grid = Vec::new();
    loop {
        let line = match lines.next() {
            Some(line) if !line.is_empty() => line,
            _ => break,
        };
        grid.push(line.chars().collect());
    }

    // Parse directions
    let mut directions = Vec::new();
    for line in lines {
        for c in line.chars() {
            if let Some(dir) = Direction::from_char(c) {
                directions.push(dir);
            }
        }
    }

    println!("{:?}", directions[directions.len() - 1]);

    (grid, directions)
}

fn find_robot(grid: &mut Grid) -> Position {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                grid[i][j] = '.';
                return (i, j);
            }
        }
    }
    panic!("No robot found in grid");
}

fn display_grid(grid: &Grid, robot_pos: Position) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if (i, j) == robot_pos {
                print!("@");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
    println!();
}

fn calculate_score(grid: &Grid) -> usize {
    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                sum += 100 * i + j;
            }
        }
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let (grid, directions) = parse_input(&input);
    println!("Grid size: {}x{}", grid[0].len(), grid.len());
    println!("Number of directions: {}", directions.len());

    part1(&grid, &directions);
    part2(&grid, &directions);
}

fn part1(grid: &Grid, directions: &[Direction]) {
    let mut grid = grid.clone();
    let mut robot_pos = find_robot(&mut grid);

    for &direction in directions {
        let (dy, dx) = direction.get_delta();
        let new_pos = (
            (robot_pos.0 as i64 + dy) as usize,
            (robot_pos.1 as i64 + dx) as usize,
        );

        match grid[new_pos.0][new_pos.1] {
            '#' => continue,
            'O' => {
                let (dy, dx) = direction.get_delta();
                let mut current = (new_pos.0 as i64, new_pos.1 as i64);

                while grid[current.0 as usize][current.1 as usize] != '#'
                    && grid[current.0 as usize][current.1 as usize] != '.'
                {
                    current = (current.0 + dy, current.1 + dx);
                }

                if grid[current.0 as usize][current.1 as usize] != '#' {
                    grid[current.0 as usize][current.1 as usize] = 'O';
                    grid[new_pos.0][new_pos.1] = '.';
                    robot_pos = new_pos;
                }
            }
            '.' => robot_pos = new_pos,
            _ => unreachable!("Invalid grid cell"),
        }
    }

    println!("{}", calculate_score(&grid));
}

fn part2(grid: &Grid, directions: &[Direction]) {
    let mut new_grid = Vec::new();

    for i in 0..grid.len() {
        let mut row = Vec::new();
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '@' => {
                    row.extend(vec!['@', '.']);
                }
                '#' => {
                    row.extend(vec!['#', '#']);
                }
                '.' => {
                    row.extend(vec!['.', '.']);
                }
                'O' => {
                    row.extend(vec!['[', ']']);
                }
                _ => unreachable!("Invalid grid cell"),
            }
        }
        new_grid.push(row);
    }

    let mut robot_pos = (0, 0);

    for i in 0..new_grid.len() {
        for j in 0..new_grid[i].len() {
            if new_grid[i][j] == '@' {
                robot_pos = (i, j);
                new_grid[i][j] = '.';
                break;
            }
        }
    }

    for dir in directions {
        // println!("{:?}", dir);

        let (dy, dx) = dir.get_delta();

        // running into a wall
        if new_grid[robot_pos.0 + dy as usize][robot_pos.1 + dx as usize] == '#' {
            // display_grid(&new_grid, robot_pos);
            continue;
        }

        // running into an open space
        if new_grid[robot_pos.0 + dy as usize][robot_pos.1 + dx as usize] == '.' {
            robot_pos = (robot_pos.0 + dy as usize, robot_pos.1 + dx as usize);
            // display_grid(&new_grid, robot_pos);
            continue;
        }

        // running into a box
        if new_grid[robot_pos.0 + dy as usize][robot_pos.1 + dx as usize] == '['
            || new_grid[robot_pos.0 + dy as usize][robot_pos.1 + dx as usize] == ']'
        {
            let initial_box =
                if new_grid[robot_pos.0 + dy as usize][robot_pos.1 + dx as usize] == '[' {
                    (
                        (robot_pos.0 + dy as usize, robot_pos.1 + dx as usize),
                        (
                            robot_pos.0 + dy as usize,
                            (robot_pos.1 as i64 + dx + 1) as usize,
                        ),
                    )
                } else {
                    (
                        (
                            robot_pos.0 + dy as usize,
                            (robot_pos.1 as i64 + dx - 1) as usize,
                        ),
                        (robot_pos.0 + dy as usize, robot_pos.1 + dx as usize),
                    )
                };

            let all_boxes = get_all_boxes(initial_box, 0, *dir, &new_grid);

            // for every box, check that there's nothing obstructing the boxes path
            if all_boxes.iter().all(|(_, (left, right))| {
                new_grid[left.0 + dy as usize][left.1 + dx as usize] != '#'
                    && new_grid[right.0 + dy as usize][right.1 + dx as usize] != '#'
            }) {
                for (_, (left, right)) in all_boxes.iter().rev() {
                    new_grid[left.0][left.1] = '.';
                    new_grid[right.0][right.1] = '.';
                    new_grid[right.0 + dy as usize][right.1 + dx as usize] = ']';
                    new_grid[left.0 + dy as usize][left.1 + dx as usize] = '[';
                }

                robot_pos = (robot_pos.0 + dy as usize, robot_pos.1 + dx as usize);
            }
        }

        // println!("After");
        // display_grid(&new_grid, robot_pos);
        // validate_grid(&new_grid);


        // println!();
    }

    // println!("{:?}", new_grid);

    let mut sum = 0;

    for i in 0..new_grid.len() {
        for j in 0..new_grid[i].len() {
            if new_grid[i][j] == '[' {
                sum += 100 * i + j;
            }
        }
    }

    println!("{}", sum);
}

fn get_all_boxes(
    initial_box: ((usize, usize), (usize, usize)),
    depth: usize,
    dir: Direction,
    grid: &Grid,
) -> Vec<(usize, ((usize, usize), (usize, usize)))> {
    let mut boxes = vec![(depth, initial_box)];

    let (left, right) = initial_box;

    assert!(
        left.0 == right.0
            && left.1 == right.1 - 1
            && grid[left.0][left.1] == '['
            && grid[right.0][right.1] == ']',
        "Invalid box: left={:?} ({}), right={:?} ({})",
        left,
        grid[left.0][left.1],
        right,
        grid[right.0][right.1]
    );

    let (dy, dx) = dir.get_delta();

    match dir {
        Direction::Up | Direction::Down => {
            if grid[left.0 + dy as usize][left.1 + dx as usize] == '[' {
                let new_boxes = get_all_boxes(
                    (
                        (left.0 + dy as usize, left.1 + dx as usize),
                        (right.0 + dy as usize, right.1 + dx as usize),
                    ),
                    depth + 1,
                    dir,
                    grid,
                );

                let filtered_boxes: Vec<_> = new_boxes
                    .into_iter()
                    .filter(|new_box| !boxes.contains(new_box))
                    .collect();

                boxes.extend(filtered_boxes);
            }

            if grid[left.0 + dy as usize][left.1 + dx as usize] == ']' {
                let new_boxes = get_all_boxes(
                    (
                        (left.0 + dy as usize, (left.1 as i64 + dx - 1) as usize),
                        (right.0 + dy as usize, (right.1 as i64 + dx - 1) as usize),
                    ),
                    depth + 1,
                    dir,
                    grid,
                );

                let filtered_boxes: Vec<_> = new_boxes
                    .into_iter()
                    .filter(|new_box| !boxes.contains(new_box))
                    .collect();

                boxes.extend(filtered_boxes);
            }

            if grid[right.0 + dy as usize][right.1 + dx as usize] == '[' {
                let new_boxes: Vec<_> = get_all_boxes(
                    (
                        (left.0 + dy as usize, (left.1 as i64 + dx + 1) as usize),
                        (right.0 + dy as usize, (right.1 as i64 + dx + 1) as usize),
                    ),
                    depth+1,
                    dir,
                    grid,
                )
                .into_iter()
                .filter(|new_box| !boxes.contains(new_box))
                .collect();

                boxes.extend(new_boxes);
            }
        }
        Direction::Left => {
            if grid[left.0 + dy as usize][left.1 + dx as usize] == ']' {
                let new_boxes: Vec<_> = get_all_boxes(
                    ((left.0, left.1 - 2), (right.0, right.1 - 2)),
                    depth + 1,
                    dir,
                    grid,
                )
                .into_iter()
                .filter(|new_box| !boxes.contains(new_box))
                .collect();


                boxes.extend(new_boxes);
            }
        }
        Direction::Right => {
            if grid[right.0 + dy as usize][right.1 + dx as usize] == '[' {
                let new_boxes: Vec<_> = get_all_boxes(
                    ((left.0, left.1 + 2), (right.0, right.1 + 2)),
                    depth+ 1,
                    dir,
                    grid,
                )
                .into_iter()
                .filter(|new_box| !boxes.contains(new_box))
                .collect();

                boxes.extend(new_boxes);
            }
        }
    }

    boxes.sort_by(|a, b| a.0.cmp(&b.0));

    boxes
}


fn validate_grid(grid: &Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                continue;
            }

            if grid[i][j] == '.'{ 
                continue;
            }

            if grid[i][j] == '[' {
                assert!(grid[i][j + 1] == ']', "Invalid box: left={:?} ({})", i, j);
            }

            if grid[i][j] == ']' {
                assert!(grid[i][j - 1] == '[', "Invalid box: right={:?} ({})", i, j);
            }
        }
    }
}