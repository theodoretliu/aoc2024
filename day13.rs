use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct GameData {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_input() -> Vec<GameData> {
    let mut games = Vec::new();

    let mut i = 0;

    let lines: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    while i < lines.len() {
        if lines[i].is_empty() {
            i += 1;
            continue;
        }

        // Parse Button A
        let button_a = &lines[i];
        let coords: Vec<&str> = button_a
            .split(": ")
            .nth(1)
            .unwrap()
            .trim_start_matches("X+")
            .trim_start_matches("X=")
            .split(", Y")
            .collect();
        let x1 = coords[0].parse::<i64>().unwrap();
        let y1 = coords[1]
            .trim_start_matches("+")
            .trim_start_matches("=")
            .parse::<i64>()
            .unwrap();

        // Parse Button B
        let button_b = &lines[i + 1];
        let coords: Vec<&str> = button_b
            .split(": ")
            .nth(1)
            .unwrap()
            .trim_start_matches("X+")
            .trim_start_matches("X=")
            .split(", Y")
            .collect();
        let x2 = coords[0].parse::<i64>().unwrap();
        let y2 = coords[1]
            .trim_start_matches("+")
            .trim_start_matches("=")
            .parse::<i64>()
            .unwrap();

        // Parse Prize
        let prize = &lines[i + 2];
        let coords: Vec<&str> = prize
            .split(": ")
            .nth(1)
            .unwrap()
            .trim_start_matches("X+")
            .trim_start_matches("X=")
            .split(", Y")
            .collect();
        let final_x = coords[0].parse::<i64>().unwrap();
        let final_y = coords[1]
            .trim_start_matches("+")
            .trim_start_matches("=")
            .parse::<i64>()
            .unwrap();

        games.push(GameData {
            button_a: (x1, y1),
            button_b: (x2, y2),
            prize: (final_x, final_y),
        });

        i += 4; // Move to next group (3 lines + empty line)
    }

    games
}

fn main() {
    let games = parse_input();

    part1(&games);

    let modified_games: Vec<GameData> = games
        .iter()
        .map(|g| GameData {
            button_a: g.button_a,
            button_b: g.button_b,
            prize: (g.prize.0 + 10000000000000, g.prize.1 + 10000000000000),
        })
        .collect();

    part1(&modified_games);
}

fn part1(games: &[GameData]) {
    let mut sum = 0;

    for game in games {
        let b_denom = game.button_b.0 * game.button_a.1 - game.button_a.0 * game.button_b.1;

        if b_denom == 0 {
            continue;
        }

        let b_num = game.prize.0 * game.button_a.1 - game.button_a.0 * game.prize.1;

        if b_num % b_denom != 0 {
            continue;
        }

        let b_res = b_num / b_denom;

        let a_num = game.prize.0 - game.button_b.0 * b_res;

        if a_num % game.button_a.0 != 0 {
            continue;
        }

        let a_res = a_num / game.button_a.0;

        sum += a_res * 3 + b_res;
    }

    println!("{}", sum);
}
