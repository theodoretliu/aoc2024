use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[i64]) {
    fn process(numbers: &[i64]) -> Vec<i64> {
        let mut working = Vec::new();

        for number in numbers {
            let num_digits = number.to_string().len();

            if *number == 0 {
                working.push(vec![1]);
            } else if num_digits % 2 == 0 {
                working.push(vec![
                    number.to_string()[..num_digits / 2].parse().unwrap(),
                    number.to_string()[num_digits / 2..].parse().unwrap(),
                ]);
            } else {
                working.push(vec![number * 2024]);
            }
        }

        let mut result = Vec::new();

        for vec in working {
            result.extend(vec);
        }

        return result;
    }

    let mut new = numbers.to_vec();

    for _ in 0..25 {
        new = process(&new);
    }

    println!("{:?}", new.len());
}

fn part2(numbers: &[i64]) {
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();

    fn solve(num: i64, remaining_depth: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
        if remaining_depth == 0 {
            return 1;
        }

        if cache.contains_key(&(num, remaining_depth)) {
            return cache[&(num, remaining_depth)];
        }

        let result = {
            if num == 0 {
                solve(1, remaining_depth - 1, cache)
            } else {
                let length = num.to_string().len();

                if length % 2 == 0 {
                    solve(
                        num.to_string()[..length / 2].parse().unwrap(),
                        remaining_depth - 1,
                        cache,
                    ) + solve(
                        num.to_string()[length / 2..].parse().unwrap(),
                        remaining_depth - 1,
                        cache,
                    )
                } else {
                    solve(num * 2024, remaining_depth - 1, cache)
                }
            }
        };

        cache.insert((num, remaining_depth), result);

        return result;
    }

    let mut sum = 0;
    for number in numbers {
        sum += solve(*number, 75, &mut cache);
    }

    println!("{:?}", sum);
}
