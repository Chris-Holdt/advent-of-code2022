use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Lines = io::Lines<io::BufReader<File>>;
fn main() {
    let lines = get_puzzle_input("./input.txt");
    // let packet = sample_input();
    let l: Vec<String>;
    match lines.collect() {
        Ok(val) => l = val,
        Err(err) => {
            println!("Error getting input {:?}", err);
            return;
        }
    }

    let packet = l[0].clone();
    let char_count = process(packet);
    println!("Characters before first marker: {}", char_count);
}

fn process(packet: String) -> i32 {
    let chars: Vec<char> = packet.chars().collect();

    let mut last_four: Vec<char> = vec![];
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if contains(&last_four, &c) {
            last_four = vec![];
        }

        last_four.push(c);

        if last_four.len() == 4 {
            return (i as i32) + 1;
        }

        i += 1;
    }
    0
}

fn contains(v: &Vec<char>, c: &char) -> bool {
    for curr_char in v {
        if curr_char == c {
            return true;
        }
    }

    false
}

fn sample_input() -> String {
    // First marker after char 6
    return String::from("nppdvjthqldpwncqszvftbrmjlhg");
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
