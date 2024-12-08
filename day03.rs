fn check_mul_pattern(s: &str, i: usize) -> Option<(u32, u32)> {
    let s = s.get(i..)?.strip_prefix("mul(")?;

    // Find first number and comma
    let (num1_str, rest) = s.split_once(',')?;
    let num1: u32 = num1_str.parse().ok()?;
    if num1_str.len() > 3 {
        return None;
    }

    // Find second number and closing paren
    let (num2_str, _rest) = rest.split_once(')')?;
    let num2: u32 = num2_str.parse().ok()?;
    if num2_str.len() > 3 {
        return None;
    }

    Some((num1, num2))
}

fn main() {
    let s = include_str!("day03.txt");
    let s_replaced = s.replace('\n', "");

    let mut count = 0;
    let mut sum = 0;

    for i in 0..s_replaced.len() {
        if let Some((num1, num2)) = check_mul_pattern(&s_replaced, i) {
            sum += num1 * num2;
            count += 1;
        }
    }

    println!("{} {}", sum, count);

    part2(&s_replaced);
}

fn check_do_pattern(s: &str, i: usize) -> Option<bool> {
    let s = s.get(i..)?.strip_prefix("do()")?;
    Some(true)
}

fn check_dont_pattern(s: &str, i: usize) -> Option<bool> {
    let s = s.get(i..)?.strip_prefix("don't()")?;
    Some(true)
}

fn part2(s: &str) {
    let mut count = 0;
    let mut sum = 0;

    let mut enabled = true;

    for i in 0..s.len() {
        if let Some(num) = check_do_pattern(&s, i) {
            enabled = true;
        } else if let Some(num) = check_dont_pattern(&s, i) {
            enabled = false;
        } else if let Some((num1, num2)) = check_mul_pattern(&s, i) {
            if enabled {
                sum += num1 * num2;
                count += 1;
            }
        }
    }

    println!("{} {}", sum, count);
}
