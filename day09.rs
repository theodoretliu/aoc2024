use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
enum Entry {
    Block { id: u64, size: usize },
    Gap { size: usize },
}

fn main() {
    // Read input line and convert to digits
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
    // Initialize file system with blocks and gaps
    let mut file_system = Vec::new();
    let mut block_id = 0;

    for (i, &digit) in digits.iter().enumerate() {
        let count = digit as usize;
        if i % 2 == 0 {
            // Add block entries
            for _ in 0..count {
                file_system.push(Some(block_id));
            }
            block_id += 1;
        } else {
            // Add gap entries
            for _ in 0..count {
                file_system.push(None);
            }
        }
    }

    // Move blocks left to fill gaps
    let mut gap_pos = 0;
    let mut block_pos = file_system.len() - 1;

    while gap_pos < block_pos {
        // Find next gap
        while gap_pos < file_system.len() && file_system[gap_pos].is_some() {
            gap_pos += 1;
        }
        if gap_pos >= block_pos {
            break;
        }

        // Find next block from right
        while block_pos > gap_pos && file_system[block_pos].is_none() {
            block_pos -= 1;
        }
        if block_pos <= gap_pos {
            break;
        }

        // Move block to gap
        file_system[gap_pos] = file_system[block_pos];
        file_system[block_pos] = None;
    }

    // Calculate score
    let mut sum = 0;
    for (pos, block) in file_system.iter().enumerate() {
        if let Some(id) = block {
            sum += pos as u64 * *id as u64;
        }
    }

    println!("{}", sum);
}

fn part2(digits: &[u32]) {
    // Initialize file system with blocks and gaps
    let mut file_system = Vec::new();
    let mut block_id = 0;
    let mut is_block = true;

    // Build initial file system
    for &digit in digits {
        let size = digit as usize;
        if is_block {
            file_system.push(Entry::Block { id: block_id, size });
            block_id += 1;
        } else {
            file_system.push(Entry::Gap { size });
        }
        is_block = !is_block;
    }

    // Process blocks from right to left
    for current_id in (0..block_id).rev() {
        // Find current block
        let block_pos = file_system
            .iter()
            .position(|entry| matches!(entry, Entry::Block { id, .. } if *id == current_id))
            .unwrap();

        let block_size = match file_system[block_pos] {
            Entry::Block { size, .. } => size,
            _ => unreachable!(),
        };

        // Find leftmost gap that can fit this block
        let mut gap_pos = None;
        for (pos, entry) in file_system[..block_pos].iter().enumerate() {
            if let Entry::Gap { size } = entry {
                if *size >= block_size {
                    gap_pos = Some(pos);
                    break;
                }
            }
        }

        if let Some(pos) = gap_pos {
            // Get gap size
            let gap_size = match file_system[pos] {
                Entry::Gap { size } => size,
                _ => unreachable!(),
            };

            // Remove original block and replace with gap
            file_system[block_pos] = Entry::Gap { size: block_size };

            // Update gap
            file_system.remove(pos);
            let remaining_size = gap_size - block_size;
            if remaining_size > 0 {
                file_system.insert(
                    pos,
                    Entry::Gap {
                        size: remaining_size,
                    },
                );
            }

            // Insert block in new position
            file_system.insert(
                pos,
                Entry::Block {
                    id: current_id,
                    size: block_size,
                },
            );
        }
    }

    // Calculate score
    let mut sum = 0;
    let mut current_pos = 0;

    for entry in &file_system {
        match entry {
            Entry::Block { id, size } => {
                for offset in 0..*size {
                    sum += (current_pos + offset as u64) * id;
                }
                current_pos += *size as u64;
            }
            Entry::Gap { size } => {
                current_pos += *size as u64;
            }
        }
    }

    println!("{}", sum);
}
