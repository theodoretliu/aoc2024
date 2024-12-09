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

fn part1(digits: &Vec<u32>) {
    let mut file_system = Vec::new();

    let mut block = true;
    let mut block_count = 0;

    for digit in digits {
        if block {
            for i in 0..*digit {
                file_system.push(Some(block_count));
            }

            block_count += 1;
            block = false;
        } else {
            for i in 0..*digit {
                file_system.push(None);
            }
            block = true;
        }
    }

    // println!("{:?}", file_system);

    let mut first_none = file_system.iter().position(|x| x.is_none()).unwrap();

    let mut idx = file_system.len() - 1;

    while first_none <= idx {
        assert!(file_system[first_none].is_none());
        assert!(file_system[idx].is_some());

        file_system[first_none] = Some(file_system[idx].unwrap());
        file_system[idx] = None;

        while first_none < file_system.len() && file_system[first_none].is_some() {
            first_none += 1;
        }

        while idx > 0 && file_system[idx].is_none() {
            idx -= 1;
        }
    }

    let mut find_first_none = file_system.iter().position(|x| x.is_none()).unwrap();

    for i in find_first_none..file_system.len() {
        assert!(file_system[i].is_none());
    }

    // println!("{:?}", file_system);
    let mut sum = 0;

    for i in 0..file_system.len() {
        if file_system[i].is_some() {
            sum += i as u64 * file_system[i].unwrap();
        } else {
            break;
        }
    }

    println!("{}", sum);
}
