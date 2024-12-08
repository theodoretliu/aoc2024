use std::io::{self, BufRead};

fn main() {
    let entries = parse_input();
    println!("{}", part1(&entries));
    println!("{}", part2(&entries));
}

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    let stdin = io::stdin();
    let mut entries = Vec::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let mut parts = line.split(':');

        let key = match parts.next().and_then(|s| s.trim().parse::<u64>().ok()) {
            Some(k) => k,
            None => continue,
        };

        let values: Vec<u64> = match parts.next() {
            Some(v) => v
                .trim()
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect(),
            None => continue,
        };

        entries.push((key, values));
    }

    entries
}

fn evaluate_entry(key: u64, values: &[u64], allow_concat: bool) -> bool {
    if values.is_empty() {
        return false;
    }

    let mut available_nums = vec![values[0]];
    let remaining_values = &values[1..];

    for &value in remaining_values {
        let mut new_nums = Vec::new();

        for &num in &available_nums {
            // Basic arithmetic operations
            new_nums.push(num + value);
            new_nums.push(num * value);

            // Optional concatenation
            if allow_concat {
                let concat_str = format!("{}{}", num, value);
                if let Ok(concat_num) = concat_str.parse::<u64>() {
                    new_nums.push(concat_num);
                }
            }
        }

        available_nums = new_nums;
    }

    available_nums.contains(&key)
}

fn solve(entries: &[(u64, Vec<u64>)], allow_concat: bool) -> u64 {
    let mut sum = 0;

    for (key, values) in entries {
        if evaluate_entry(*key, values, allow_concat) {
            sum += key;
        }
    }

    sum
}

fn part1(entries: &[(u64, Vec<u64>)]) -> u64 {
    solve(entries, false)
}

fn part2(entries: &[(u64, Vec<u64>)]) -> u64 {
    solve(entries, true)
}
