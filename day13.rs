use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct GameData {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_coordinates(line: &str) -> (i64, i64) {
    let coords = line.split(": ").nth(1).unwrap();
    let coords = coords.trim_start_matches("X+").trim_start_matches("X=");
    let mut parts = coords.split(", Y");

    let x = parts.next().unwrap().parse::<i64>().unwrap();
    let y = parts
        .next()
        .unwrap()
        .trim_start_matches("+")
        .trim_start_matches("=")
        .parse::<i64>()
        .unwrap();

    (x, y)
}

fn parse_input() -> Vec<GameData> {
    let mut games = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        // Parse three consecutive lines for each game
        let button_a = parse_coordinates(&line);
        let button_b = parse_coordinates(&lines.next().unwrap().unwrap());
        let prize = parse_coordinates(&lines.next().unwrap().unwrap());

        games.push(GameData {
            button_a,
            button_b,
            prize,
        });

        // Skip empty line between games
        let _ = lines.next();
    }

    games
}

fn main() {
    let games = parse_input();
    part1(&games);

    // Create modified games with offset prize coordinates
    let mut modified_games = Vec::with_capacity(games.len());
    let offset = 10000000000000;

    for game in &games {
        modified_games.push(GameData {
            button_a: game.button_a,
            button_b: game.button_b,
            prize: (game.prize.0 + offset, game.prize.1 + offset),
        });
    }

    part1(&modified_games);
}

fn solve_game(game: &GameData) -> Option<i64> {
    // Calculate denominator for b coefficient
    let b_denom = game.button_b.0 * game.button_a.1 - game.button_a.0 * game.button_b.1;
    if b_denom == 0 {
        return None;
    }

    // Calculate numerator and check divisibility
    let b_num = game.prize.0 * game.button_a.1 - game.button_a.0 * game.prize.1;
    if b_num % b_denom != 0 {
        return None;
    }

    let b_res = b_num / b_denom;

    // Calculate a coefficient
    let a_num = game.prize.0 - game.button_b.0 * b_res;
    if a_num % game.button_a.0 != 0 {
        return None;
    }

    let a_res = a_num / game.button_a.0;
    Some(a_res * 3 + b_res)
}

fn part1(games: &[GameData]) {
    let mut sum = 0;
    for game in games {
        if let Some(result) = solve_game(game) {
            sum += result;
        }
    }
    println!("{}", sum);
}
