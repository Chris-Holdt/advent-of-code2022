use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Lines = io::Lines<io::BufReader<File>>;

fn main() {
    let lines = get_puzzle_input("./input.txt");

    let mut accumulator = 0;

    for line in lines {
        if let Ok(pair) = line {
            let pair_vec: Vec<&str> = pair.split(",").collect();
            if does_encompass(pair_vec) {
                accumulator += 1;
            }
        }
    }

    println!("Fully encompasses {}", accumulator);
}

fn does_encompass(pair: Vec<&str>) -> bool {
    let elf_one_range: Vec<&str> = pair[0].split("-").collect();
    let elf_two_range: Vec<&str> = pair[1].split("-").collect();

    let elf_1_up: i32;
    let elf_1_down: i32;
    let elf_2_up: i32;
    let elf_2_down: i32;

    match elf_one_range[0].parse::<i32>() {
        Ok(num) => elf_1_up = num,
        Err(why) => panic!("{:?}", why),
    }

    match elf_one_range[1].parse::<i32>() {
        Ok(num) => elf_1_down = num,
        Err(why) => panic!("{:?}", why),
    }

    match elf_two_range[0].parse::<i32>() {
        Ok(num) => elf_2_up = num,
        Err(why) => panic!("{:?}", why),
    }

    match elf_two_range[1].parse::<i32>() {
        Ok(num) => elf_2_down = num,
        Err(why) => panic!("{:?}", why),
    }

    let mut encompass = true;

    if (elf_1_up > elf_2_up
        && elf_1_down > elf_2_down
        && elf_1_up > elf_2_down
        && elf_1_down > elf_2_up)
        || (elf_1_up < elf_2_up
            && elf_1_down < elf_2_down
            && elf_1_up < elf_2_down
            && elf_1_down < elf_2_up)
    {
        encompass = false;
    }

    /*
        if (elf_1_up <= elf_2_up && elf_1_down >= elf_2_down)
            || (elf_1_up >= elf_2_up && elf_1_down <= elf_2_down)
        {
            encompass = true;
        }
    */
    /*
        if (elf_one_range[0] >= elf_two_range[0] && elf_one_range[1] >= elf_two_range[1])
            || (elf_one_range[0] <= elf_two_range[0] && elf_one_range[1] <= elf_two_range[1])
        {
            encompass = true;
        }
    */
    encompass
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
