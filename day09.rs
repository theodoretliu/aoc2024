use std::io::{self, BufRead};

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
