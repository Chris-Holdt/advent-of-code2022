use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Selection = i32;
const ROCK: Selection = 1;
const PAPER: Selection = 2;
const SCISSORS: Selection = 3;
const NOSCORE: Selection = 5;

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
                let opponent: Selection;
                let me: Selection;

                match split[0] {
                    "A" => opponent = ROCK,
                    "B" => opponent = PAPER,
                    "C" => opponent = SCISSORS,
                    _ => opponent = NOSCORE,
                }

                match split[1] {
                    "X" => {
                        score += 1;
                        me = ROCK;
                    }
                    "Y" => {
                        score += 2;
                        me = PAPER;
                    }
                    "Z" => {
                        score += 3;
                        me = SCISSORS;
                    }
                    _ => me = NOSCORE,
                }

                if me == NOSCORE {
                    panic!("I have no score. Can't deal with this")
                }

                if opponent == NOSCORE {
                    panic!("Opponent has no score. Can't deal with this")
                }

                if me == opponent {
                    score += 3;
                    continue;
                }

                if me == ROCK && opponent != PAPER {
                    score += 6;
                    continue;
                }

                if me == PAPER && opponent != SCISSORS {
                    score += 6;
                    continue;
                }

                if me == SCISSORS && opponent != ROCK {
                    score += 6;
                    continue;
                }
            }
        }
    }

    score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
