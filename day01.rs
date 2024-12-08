use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                first_numbers.push(num1);
                second_numbers.push(num2);
            }
        }
    }

    first_numbers.sort();
    second_numbers.sort();

    let mut sum = 0;

    for i in 0..first_numbers.len() {
        sum += (first_numbers[i] - second_numbers[i]).abs();
    }

    let mut sum2 = 0;

    for i in 0..first_numbers.len() {
        let count = second_numbers
            .iter()
            .filter(|&x| *x == first_numbers[i])
            .count() as i32;
        sum2 += first_numbers[i] * count;
    }

    println!("{}", sum2);

    println!("{}", sum);
}
