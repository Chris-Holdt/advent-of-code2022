use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Lines = io::Lines<io::BufReader<File>>;

fn main() {
    let lines = get_puzzle_input("./input.txt");
    let sum = process(lines);
    println!("Sum of priority: {}", sum)
    // _ = find_duplicate(String::from("aaabbb"))
}

fn process(lines: Lines) -> i32 {
    let mut accumulator = 0;

    let mut triple: Vec<String> = vec![];
    for line in lines {
        if let Ok(pack) = line {
            triple.push(pack)
        }

        if triple.len() == 3 {
            if let Some(triplicate) = find_triplicate(triple) {
                let priority = get_item_priority(triplicate);
                accumulator += priority;
            }

            triple = vec![];
        }
    }

    accumulator
}

fn find_triplicate(packs: Vec<String>) -> Option<char> {
    let mut pack_split: Vec<Vec<char>> = vec![];

    for pack in packs {
        let split: Vec<char> = pack.chars().collect();
        pack_split.push(split)
    }

    let mut found_char = None;

    for pack_one_char in pack_split[0].clone() {
        for pack_two_char in pack_split[1].clone() {
            if pack_two_char != pack_one_char {
                continue;
            }

            for pack_three_char in pack_split[2].clone() {
                if pack_two_char == pack_three_char {
                    found_char = Some(pack_three_char)
                }
            }
        }
    }

    found_char
}

fn get_item_priority(item: char) -> i32 {
    let ascii = item as i32;

    if ascii >= 97 {
        ascii - 96
    } else {
        ascii - 38
    }
}

fn get_puzzle_input(filename: &str) -> Lines {
    match read_lines(filename) {
        Err(why) => panic!("{:?}", why),
        Ok(lines) => lines,
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
