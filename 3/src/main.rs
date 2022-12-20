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

    for line in lines {
        if let Ok(pack) = line {
            if let Some(duplicate) = find_duplicate(pack) {
                let priority = get_item_priority(duplicate);
                accumulator += priority;
            }
        }
    }

    accumulator
}

fn find_duplicate(pack: String) -> Option<char> {
    let split: Vec<char> = pack.chars().collect();
    let (comp_one, comp_two) = split.split_at(split.len() / 2);

    let mut duplicate = None;
    for (_, comp_one_item) in comp_one.iter().enumerate() {
        for (_, comp_two_item) in comp_two.iter().enumerate() {
            if comp_one_item == comp_two_item {
                duplicate = Some(comp_one_item.clone());
            }
        }
    }

    duplicate
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
