use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut entries: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim().parse::<u64>().unwrap();
        let values: Vec<u64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        entries.push((key, values));
    }

    println!("{}", part1(&entries));
    println!("{}", part2(&entries));
}

fn part1(entries: &[(u64, Vec<u64>)]) -> u64 {
    let mut count = 0;

    fn evaluate_entry(key: u64, values: &[u64]) -> bool {
        let mut avail = vec![values[0]];

        for i in 1..values.len() {
            let mut new_avail = Vec::new();

            for num in avail {
                new_avail.push(num + values[i]);
                new_avail.push(num * values[i]);
            }

            avail = new_avail;
        }

        if avail.contains(&key) {
            return true;
        }

        false
    }

    for (key, values) in entries {
        if evaluate_entry(*key, values) {
            count += key;
        }
    }

    count
}

fn part2(entries: &[(u64, Vec<u64>)]) -> u64 {
    let mut count = 0;

    fn evaluate_entry(key: u64, values: &[u64]) -> bool {
        let mut avail = vec![values[0]];

        for i in 1..values.len() {
            let mut new_avail = Vec::new();

            for num in avail {
                new_avail.push(num + values[i]);
                new_avail.push(num * values[i]);
                new_avail.push(format!("{}{}", num, values[i]).parse::<u64>().unwrap());
            }

            avail = new_avail;
        }

        if avail.contains(&key) {
            return true;
        }

        false
    }

    for (key, values) in entries {
        if evaluate_entry(*key, values) {
            count += key;
        }
    }

    count
}
