fn check_mul_pattern(s: &str, i: usize) -> Option<(u32, u32)> {
    let s = s.get(i..)?.strip_prefix("mul(")?;

    // Parse first number before comma
    let (num1_str, rest) = s.split_once(',')?;
    let num1: u32 = num1_str.parse().ok()?;
    if num1_str.len() > 3 {
        return None;
    }

    // Parse second number before closing paren
    let (num2_str, _rest) = rest.split_once(')')?;
    let num2: u32 = num2_str.parse().ok()?;
    if num2_str.len() > 3 {
        return None;
    }

    Some((num1, num2))
}

fn main() {
    let input = include_str!("day03.txt");
    let input = input.replace('\n', "");

    let (sum, count) = process_multiplications(&input);
    println!("{} {}", sum, count);

    process_controlled_multiplications(&input);
}

fn check_do_pattern(s: &str, i: usize) -> Option<()> {
    s.get(i..)?.strip_prefix("do()")?;
    Some(())
}

fn check_dont_pattern(s: &str, i: usize) -> Option<()> {
    s.get(i..)?.strip_prefix("don't()")?;
    Some(())
}

fn process_multiplications(s: &str) -> (u32, u32) {
    let mut count = 0;
    let mut sum = 0;

    for i in 0..s.len() {
        if let Some((num1, num2)) = check_mul_pattern(s, i) {
            sum += num1 * num2;
            count += 1;
        }
    }

    (sum, count)
}

fn process_controlled_multiplications(s: &str) {
    let mut count = 0;
    let mut sum = 0;
    let mut enabled = true;

    for i in 0..s.len() {
        if check_do_pattern(s, i).is_some() {
            enabled = true;
        } else if check_dont_pattern(s, i).is_some() {
            enabled = false;
        } else if let Some((num1, num2)) = check_mul_pattern(s, i) {
            if enabled {
                sum += num1 * num2;
                count += 1;
            }
        }
    }

    println!("{} {}", sum, count);
}
