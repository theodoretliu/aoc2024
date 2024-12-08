use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut numbers = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        if !nums.is_empty() {
            numbers.push(nums);
        }
    }

    let mut count = 0;

    for number in &numbers {
        if number
            .windows(2)
            .all(|w| w[0] > w[1] && (1..=3).contains(&(w[0] - w[1])))
            || number
                .windows(2)
                .all(|w| w[0] < w[1] && (1..=3).contains(&(w[1] - w[0])))
        {
            count += 1;
        }
    }

    let mut count2 = 0;

    for number in &numbers {
        if is_valid(&number) {
            count2 += 1;
        } else {
            for i in 0..number.len() {
                let mut modified = number.clone();
                modified.remove(i);
                if is_valid(&modified) {
                    count2 += 1;
                    break;
                }
            }
        }
    }

    println!("{:?}", count);
    println!("{:?}", count2);
}

fn is_valid(number: &Vec<i32>) -> bool {
    number
        .windows(2)
        .all(|w| w[0] > w[1] && (1..=3).contains(&(w[0] - w[1])))
        || number
            .windows(2)
            .all(|w| w[0] < w[1] && (1..=3).contains(&(w[1] - w[0])))
}
