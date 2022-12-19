use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let calories = sum_calories("./calories.txt");

    let mut top_three: [i32; 3] = [0, 0, 0];

    for calory in calories {
        for (_, top) in top_three.iter_mut().enumerate() {
            if calory > top.clone() {
                *top = calory;
                break;
            }
        }
    }

    let total = top_three[0] + top_three[1] + top_three[2];
    println!("Top three total {}", total)
}

fn sum_calories(file: &str) -> Vec<i32> {
    let mut calories = vec![];

    if let Ok(lines) = read_lines(file) {
        let mut accumulator = 0;

        for line in lines {
            if let Ok(calory) = line {
                if calory.is_empty() {
                    calories.push(accumulator);
                    accumulator = 0;
                    continue;
                }

                if let Ok(calory_int) = calory.parse::<i32>() {
                    accumulator += calory_int;
                }
            }
        }
    }

    calories
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
