use std::collections::HashSet;

use std::io::{self, BufRead};



fn main() {
    let lines: Vec<((usize, usize), (i64, i64))> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" v=").collect();

            let pos_str = parts[0].trim_start_matches("p=");
            let pos_parts: Vec<usize> = pos_str.split(',').map(|n| n.parse().unwrap()).collect();

            let vel_parts: Vec<i64> = parts[1].split(',').map(|n| n.parse().unwrap()).collect();

            ((pos_parts[0], pos_parts[1]), (vel_parts[0], vel_parts[1]))
        })
        .collect();

    part1(&lines);
    part2(&lines);
}

fn print_grid(grid: &[Vec<HashSet<(i64, i64)>>]) {
    for row in grid {
        for cell in row {
            if cell.len() > 0 {
                print!("{}", cell.len());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn create_grid(width: usize, height: usize) -> Vec<Vec<HashSet<(i64, i64)>>> {
    let mut grid = Vec::new();

    for _i in 0..height {
        let mut row = Vec::new();
        for _j in 0..width {
            row.push(HashSet::<(i64, i64)>::new());
        }
        grid.push(row);
    }

    grid
}

fn part1(lines: &[((usize, usize), (i64, i64))]) {
    let mut grid = Vec::new();

    let width = 101;
    let height = 103;

    let num_rounds = 100;

    for _i in 0..height {
        let mut row = Vec::new();
        for _j in 0..width {
            row.push(HashSet::<(i64, i64)>::new());
        }
        grid.push(row);
    }

    for (pos, vel) in lines {
        let (x, y) = pos;
        let (dx, dy) = vel;

        grid[*y][*x].insert((*dx, *dy));
    }

    let mut final_grid = vec![vec![0; width]; height];

    for i in 0..height {
        for j in 0..width {
            for (dx, dy) in &grid[i][j] {
                let mut new_pos = (
                    (i as i64 + num_rounds * dy) % height as i64,
                    (j as i64 + num_rounds * dx) % width as i64,
                );

                if new_pos.0 < 0 {
                    new_pos.0 += height as i64;
                }
                if new_pos.1 < 0 {
                    new_pos.1 += width as i64;
                }

                final_grid[new_pos.0 as usize][new_pos.1 as usize] += 1;
            }
        }
    }

    let mut first_quadrant_count = 0;

    for i in 0..height / 2 {
        for j in 0..width / 2 {
            first_quadrant_count += final_grid[i][j];
        }
    }

    let mut second_quadrant_count = 0;

    for i in 0..height / 2 {
        for j in (width / 2 + 1)..width {
            second_quadrant_count += final_grid[i][j];
        }
    }

    let mut third_quadrant_count = 0;

    for i in height / 2 + 1..height {
        for j in width / 2 + 1..width {
            third_quadrant_count += final_grid[i][j];
        }
    }

    let mut fourth_quadrant_count = 0;

    for i in height / 2 + 1..height {
        for j in 0..width / 2 {
            fourth_quadrant_count += final_grid[i][j];
        }
    }

    println!(
        "{}",
        first_quadrant_count * second_quadrant_count * third_quadrant_count * fourth_quadrant_count
    );
}

fn checksum(grid: &[Vec<HashSet<(i64, i64)>>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut first_quadrant_count = 0;

    for i in 0..height / 2 {
        for j in 0..width / 2 {
            first_quadrant_count += grid[i][j].len();
        }
    }

    let mut second_quadrant_count = 0;

    for i in 0..height / 2 {
        for j in (width / 2 + 1)..width {
            second_quadrant_count += grid[i][j].len();
        }
    }

    let mut third_quadrant_count = 0;

    for i in height / 2 + 1..height {
        for j in width / 2 + 1..width {
            third_quadrant_count += grid[i][j].len();
        }
    }

    let mut fourth_quadrant_count = 0;

    for i in height / 2 + 1..height {
        for j in 0..width / 2 {
            fourth_quadrant_count += grid[i][j].len();
        }
    }

    first_quadrant_count * second_quadrant_count * third_quadrant_count * fourth_quadrant_count
}

fn output_grid_image(grid: &[Vec<HashSet<(i64, i64)>>], step: usize) {
    use image::{ImageBuffer, Rgb};

    let width = grid[0].len();
    let height = grid.len();

    let mut img = ImageBuffer::new(width as u32, height as u32);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let pixel = if cell.is_empty() {
                Rgb([255u8, 255u8, 255u8]) // White for empty cells
            } else {
                Rgb([0u8, 0u8, 0u8]) // Black for occupied cells
            };
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    img.save(format!("day14-images/grid_output_{}.png", step))
        .expect("Failed to save image");
}

fn create_grid_animation(grids: &[Vec<Vec<HashSet<(i64, i64)>>>], output_path: &str) {
    use gif::{Encoder, Frame, Repeat};
    use std::borrow::Cow;
    use std::fs::File;

    let width = grids[0][0].len() as u16;
    let height = grids[0].len() as u16;

    let mut image_frames = Vec::new();

    // Create frames for each grid state
    for grid in grids {
        let mut pixels = Vec::new();
        for row in grid {
            for cell in row {
                if cell.is_empty() {
                    pixels.extend_from_slice(&[255, 255, 255]); // White
                } else {
                    pixels.extend_from_slice(&[0, 0, 0]); // Black
                }
            }
        }

        let frame = Frame {
            width,
            height,
            buffer: Cow::Owned(pixels.clone()),
            delay: 10, // 100ms delay between frames
            transparent: None,
            dispose: gif::DisposalMethod::Any,
            needs_user_input: false,
            top: 0,
            left: 0,
            interlaced: false,
            palette: None,
        };

        image_frames.push(frame);
    }

    // Create and save the GIF
    let mut file = File::create(output_path).expect("Failed to create GIF file");
    let mut encoder =
        Encoder::new(&mut file, width, height, &[]).expect("Failed to create encoder");
    encoder
        .set_repeat(Repeat::Infinite)
        .expect("Failed to set repeat");

    for frame in image_frames {
        encoder.write_frame(&frame).expect("Failed to write frame");
    }
}

fn part2(lines: &[((usize, usize), (i64, i64))]) {
    let mut grid = Vec::new();

    let width = 101;
    let height = 103;

    for _i in 0..height {
        let mut row = Vec::new();
        for _j in 0..width {
            row.push(HashSet::<(i64, i64)>::new());
        }
        grid.push(row);
    }

    for (pos, vel) in lines {
        let (x, y) = pos;
        let (dx, dy) = vel;

        grid[*y][*x].insert((*dx, *dy));
    }

    let mut num_steps = 0;

    let mut grids = Vec::new();

    grids.push(grid.clone());

    let mut min_safety_factor = usize::MAX;
    let mut iteration_count = 0;

    for _ in 0..10000 {
        let mut new_grid = create_grid(width, height);

        for i in 0..height {
            for j in 0..width {
                for (dx, dy) in &grid[i][j] {
                    let mut new_pos = (
                        (i as i64 + dy) % height as i64,
                        (j as i64 + dx) % width as i64,
                    );

                    if new_pos.0 < 0 {
                        new_pos.0 += height as i64;
                    }
                    if new_pos.1 < 0 {
                        new_pos.1 += width as i64;
                    }

                    new_grid[new_pos.0 as usize][new_pos.1 as usize].insert((*dx, *dy));
                }
            }
        }

        grid = new_grid;

        num_steps += 1;

        grids.push(grid.clone());


        let safety_factor = checksum(&grid);

        if safety_factor < min_safety_factor {
            min_safety_factor = safety_factor;
            iteration_count = num_steps;
            // println!("{}", min_safety_factor);
            // println!("{}", num_steps);
            // print_grid(&grid);
        }

        // if num_steps == 100 {
        //     assert_eq!(checksum(&grid), 225521010);
        // }

        // print!("\x1B[2J\x1B[1;1H");

        // print!("{}[2J", 27 as char);

        // if num_steps > 4000 {
        //     // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //     println!("{}", num_steps);
        //     print_grid(&grid);
        //     println!();
        // }

        // io::stdout().flush().unwrap();

        // thread::sleep(Duration::from_millis(100));
    }


    println!("{}", iteration_count);
    

    // create_grid_animation(&grids, "grid_animation.gif");
}
