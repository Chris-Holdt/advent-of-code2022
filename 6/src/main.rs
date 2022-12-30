use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Lines = io::Lines<io::BufReader<File>>;
fn main() {
    // let packet = sample_input();
    let lines = get_puzzle_input("./input.txt");
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

    let mut unique_chars: Vec<char> = vec![];
    let mut i = 0;
    // println!("chars {:?}", chars);
    while i < chars.len() {
        let c = chars[i];
        // println!("Current unique list {:?}", unique_chars);
        // println!("Current char {}", c);

        let di = contains(&unique_chars, &c);
        if let Some(dupe_index) = di {
            /* println!(
                "Found dupe char {} at {}",
                unique_chars[dupe_index], dupe_index
            );
            let mut a = unique_chars.clone();
            println!("OG {:?}", a);
            let b = a.split_off(dupe_index + 1);
            println!("OG after split {:?}", a);
            println!("Split {:?}", b); */
            unique_chars = unique_chars.split_off(dupe_index + 1);
        }

        unique_chars.push(c);

        // println!("\nchar: {}, vec: {:?}\n", c, unique_chars);

        if unique_chars.len() == 14 {
            // println!("Got 14 unique chars {:?}", unique_chars);
            return (i as i32) + 1;
        }

        i += 1;
    }
    0
}

fn contains(v: &Vec<char>, c: &char) -> Option<usize> {
    let mut i = 0;
    while i < v.len() {
        let curr_char = v[i];
        if &curr_char == c {
            return Some(i);
        }

        i += 1;
    }

    None
}

fn sample_input() -> String {
    // return String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"); // 19
    // return String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"); // 23
    // return String::from("nppdvjthqldpwncqszvftbrmjlhg"); // 23
    // return String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"); // 26
    return String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"); // 29
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
