use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Block { id: u64, size: usize },
    Gap { size: usize },
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let digits: Vec<u32> = line
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    part1(&digits);
    part2(&digits);
}

fn part1(digits: &[u32]) {
    // Create blocks of Some(block_id) alternating with None values based on input digits
    let mut file_system = Vec::new();
    let mut block_id = 0;

    for (i, &digit) in digits.iter().enumerate() {
        let entries = if i % 2 == 0 {
            // Even indices: create Some(block_id) entries and increment block counter
            let entries = vec![Some(block_id); digit as usize];
            block_id += 1;
            entries
        } else {
            // Odd indices: create None entries
            vec![None; digit as usize]
        };
        file_system.extend(entries);
    }

    // Move blocks to fill gaps (None values) from left to right
    let mut first_gap = file_system.iter().position(|x| x.is_none()).unwrap();
    let mut last_block = file_system.len() - 1;

    while first_gap <= last_block {
        // Move block from right to fill gap on left
        file_system[first_gap] = file_system[last_block];
        file_system[last_block] = None;

        // Find next gap
        first_gap = match file_system[first_gap + 1..]
            .iter()
            .position(|x| x.is_none())
        {
            Some(pos) => first_gap + 1 + pos,
            None => break,
        };

        // Find next block from right
        last_block = match file_system[..last_block].iter().rposition(|x| x.is_some()) {
            Some(pos) => pos,
            None => break,
        };
    }

    // Calculate sum of (index * block_id) for all filled positions
    let sum: u64 = file_system
        .iter()
        .take_while(|x| x.is_some())
        .enumerate()
        .map(|(i, &x)| i as u64 * x.unwrap() as u64)
        .sum();

    println!("{}", sum);
}

fn part2(digits: &[u32]) {
    let mut file_system = Vec::new();
    let mut block_id = 0;

    let mut block = true;

    let mut block_map = HashMap::new();

    let mut max_block_id = 0;

    for (i, &digit) in digits.iter().enumerate() {
        if block {
            block_map.insert(block_id, i);

            if block_id > max_block_id {
                max_block_id = block_id;
            }

            file_system.push(Entry::Block {
                id: block_id,
                size: digit as usize,
            });
            block_id += 1;
        } else {
            file_system.push(Entry::Gap {
                size: digit as usize,
            });
        }
        block = !block;
    }

    // println!("{:?}", file_system);

    for i in (0..=max_block_id).rev() {
        // println!("i: {}", i);
        // println!("{:?}", file_system);
        let (
            block_idx,
            Entry::Block {
                id: _,
                size: block_size,
            },
        ) = file_system
            .iter()
            .enumerate()
            .find(|(_, x)| match x {
                Entry::Block { id, .. } => *id == i,
                _ => false,
            })
            .unwrap()
        else {
            panic!("Expected block with id {}", i);
        };

        let final_size = *block_size;

        let first_gap = file_system.iter().take(block_idx).position(|x| match x {
            Entry::Gap { size } => *size >= final_size,
            _ => false,
        });

        match first_gap {
            Some(pos) => {
                let existing_gap = match file_system[pos] {
                    Entry::Gap { size } => size,
                    _ => panic!("Expected gap at position {}", pos),
                };

                let new_gap_size = existing_gap - final_size;

                file_system[block_idx] = Entry::Gap { size: final_size };
                file_system.remove(pos);
                if new_gap_size > 0 {
                    file_system.insert(pos, Entry::Gap { size: new_gap_size });
                }

                file_system.insert(
                    pos,
                    Entry::Block {
                        id: i,
                        size: final_size,
                    },
                );
            }
            None => {
                file_system.push(Entry::Gap { size: final_size });
            }
        }
    }

    let mut sum = 0;
    let mut actual_idx = 0;
    for (i, &ref entry) in file_system.iter().enumerate() {
        match entry {
            Entry::Block { id, size } => {
                for x in 0..*size {
                    sum += (actual_idx + x as u64) * id;
                }
                actual_idx += *size as u64;
            }
            Entry::Gap { size } => {
                actual_idx += *size as u64;
            }
        }
    }

    println!("{:?}", file_system);

    println!("{}", sum);
}
