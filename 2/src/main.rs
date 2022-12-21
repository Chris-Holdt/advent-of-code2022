use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Selection = i32;
const ROCK: Selection = 1;
const PAPER: Selection = 2;
const SCISSORS: Selection = 3;
const NOSELECTION: Selection = 5;

type Outcome = i32;
const WIN: Outcome = 6;
const DRAW: Outcome = 3;
const LOSE: Outcome = 0;
const NOOUTCOME: Outcome = 5;

fn main() {
    /*
     * A, X = Rock
     * B, Y = Paper
     * C, Z = Scissors
     *
     * Rock beats Scissors
     * Paper beats Rock
     * Scissors beats Paper
     */
    let score = calc_score("./input.txt");
    println!("Score: {}", score)
}

fn calc_score(file: &str) -> i32 {
    let mut score = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(outcome) = line {
                let split: Vec<&str> = outcome.split_whitespace().collect();
                let weapon: Selection;
                let outcome: Outcome;

                match split[0] {
                    "A" => weapon = ROCK,
                    "B" => weapon = PAPER,
                    "C" => weapon = SCISSORS,
                    _ => weapon = NOSELECTION,
                }

                match split[1] {
                    "X" => outcome = LOSE,
                    "Y" => outcome = DRAW,
                    "Z" => outcome = WIN,
                    _ => outcome = NOOUTCOME,
                }

                if outcome == NOOUTCOME {
                    panic!("No Outcome. Can't deal with this")
                }

                if weapon == NOSELECTION {
                    panic!("No weapon. Can't deal with this")
                }

                let outcome_score = get_outcome_score(weapon, outcome);
            }
        }
    }

    score
}

fn get_outcome_score(weapon: Selection, outcome: Outcome) -> i32 {
    match outcome {
        WIN => {
            let weapon_score = get_weapon_score(weapon, outcome);
            6 + weapon_score
        }
        DRAW => {
            // Use the same weapon to draw
            3 + weapon
        }
        LOSE => {
            let weapon_score = get_weapon_score(weapon, outcome);
            weapon_score
        }
        _ => 0,
    }
}

fn get_weapon_score(weapon: Selection, outcome: Outcome) -> i32 {
    match weapon {
        ROCK => match outcome {
            WIN => PAPER,
            LOSE => SCISSORS,
            _ => 0,
        },
        PAPER => match outcome {
            WIN => SCISSORS,
            LOSE => ROCK,
            _ => 0,
        },
        SCISSORS => match outcome {
            WIN => ROCK,
            LOSE => PAPER,
            _ => 0,
        },
        _ => 0,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
