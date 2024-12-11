use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut numbers = Vec::new();
    for num_str in line.split_whitespace() {
        numbers.push(num_str.parse::<i64>().unwrap());
    }

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[i64]) {
    fn process(numbers: &[i64]) -> Vec<i64> {
        let mut result = Vec::new();

        for &number in numbers {
            let num_str = number.to_string();
            let num_digits = num_str.len();

            if number == 0 {
                result.push(1);
            } else if num_digits % 2 == 0 {
                let (first_half, second_half) = num_str.split_at(num_digits / 2);
                result.push(first_half.parse().unwrap());
                result.push(second_half.parse().unwrap());
            } else {
                result.push(number * 2024);
            }
        }

        result
    }

    let mut current_numbers = numbers.to_vec();
    const ITERATIONS: i32 = 25;

    for _ in 0..ITERATIONS {
        current_numbers = process(&current_numbers);
    }

    println!("{}", current_numbers.len());
}

fn part2(numbers: &[i64]) {
    fn solve(num: i64, remaining_depth: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
        // Base case
        if remaining_depth == 0 {
            return 1;
        }

        // Check cache
        if let Some(&result) = cache.get(&(num, remaining_depth)) {
            return result;
        }

        let result = if num == 0 {
            solve(1, remaining_depth - 1, cache)
        } else {
            let num_str = num.to_string();
            let num_digits = num_str.len();

            if num_digits % 2 == 0 {
                let (first_half, second_half) = num_str.split_at(num_digits / 2);
                let first_num = first_half.parse().unwrap();
                let second_num = second_half.parse().unwrap();

                solve(first_num, remaining_depth - 1, cache)
                    + solve(second_num, remaining_depth - 1, cache)
            } else {
                solve(num * 2024, remaining_depth - 1, cache)
            }
        };

        cache.insert((num, remaining_depth), result);
        result
    }

    let mut total = 0;
    const MAX_DEPTH: i64 = 75;
    let mut cache = HashMap::new();

    for &number in numbers {
        total += solve(number, MAX_DEPTH, &mut cache);
    }

    println!("{}", total);
}
