use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Lines = io::Lines<io::BufReader<File>>;
fn main() {
    let lines = get_puzzle_input("./input.txt");

    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    let mut instructions: Vec<Vec<i32>> = vec![];

    let mut stacks_raw: Vec<String> = vec![];
    let mut instructions_raw: Vec<String> = vec![];

    let mut is_instructions = false;
    for line in lines {
        if let Ok(l) = line {
            let line_chars: Vec<char> = l.chars().collect();

            if line_chars.is_empty() || line_chars[1] == '1' {
                is_instructions = true;
                continue;
            }

            if is_instructions {
                instructions_raw.push(l);
            } else {
                stacks_raw.push(l);
            }
        }
    }

    get_stacks(stacks_raw, &mut stacks);
    clean_instructions(instructions_raw, &mut instructions);
    // instructions = x, y, z
    // x = move x
    // y = from y
    // z = to z

    execute_instructions(instructions, &mut stacks);
    get_tops(stacks);
}

fn get_tops(stacks: HashMap<i32, Vec<char>>) {
    let mut tops = String::from("");

    let mut i = 0;
    while i <= stacks.len() {
        let stack_num = i as i32;

        if let Some(stack) = stacks.get(&stack_num) {
            // println!("Stack: {:?}", stack);
            let top = stack[0];

            tops.push(top);
        }

        i += 1;
    }

    println!("Tops: {}", tops);
}

fn clean_instructions(raw: Vec<String>, output: &mut Vec<Vec<i32>>) {
    for instruction in raw {
        if instruction.len() <= 0 {
            continue;
        }

        let split: Vec<&str> = instruction.split(" ").collect();
        let mut instructions: Vec<i32> = vec![];
        for i in split {
            let item = i.parse::<i32>();
            if let Ok(num) = item {
                instructions.push(num);
            }
        }

        // println!("Adding the following instructions:\n{:?}", instructions);
        output.push(instructions);
    }
}

fn execute_instructions(instructions: Vec<Vec<i32>>, stacks: &mut HashMap<i32, Vec<char>>) {
    for instruction in instructions {
        let move_amount = instruction[0];
        let from_stack = instruction[1];
        let to_stack = instruction[2];

        if let Some(stack) = stacks.get(&from_stack) {
            let mut from = stack.clone();
            let mut cache: Vec<char> = from.drain(..(move_amount as usize)).collect();
            println!(
                "Moving {} from stack {} ({:?}) to stack {}, they are {:?}",
                move_amount, from_stack, stack, to_stack, cache
            );

            println!("Stack {} is now {:?}", from_stack, from);

            if let Some(to) = stacks.get(&to_stack) {
                let new_to = to.clone();

                cache.extend(new_to);
            }
            println!("Stack {} is now {:?}", to_stack, cache);

            stacks.insert(to_stack, cache);
            stacks.insert(from_stack, from);
        }
    }

    let mut i = 0;
    while i <= stacks.len() {
        if let Some(stack) = stacks.get(&(i as i32)) {
            println!("Stack {}: {:?}", i, stack);
        }

        i += 1;
    }
}

fn get_stacks(lines: Vec<String>, stacks: &mut HashMap<i32, Vec<char>>) {
    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            if chars[i] != ' ' && chars[i] != '[' && chars[i] != ']' {
                let stack_num: i32 = (i as i32) / 4 + 1;

                match stacks.get(&stack_num) {
                    Some(stack) => {
                        let mut new_stack = stack.clone();
                        new_stack.push(chars[i]);

                        stacks.insert(stack_num, new_stack.to_vec());
                    }
                    None => {
                        stacks.insert(stack_num, vec![chars[i]]);
                    }
                }
            }
            i += 1;
        }
    }

    // Reverse stacks so the top is at the end of the vector
    // let mut i = 0;
    // while i <= stacks.len() {
    //     let stack_num = i as i32;
    //     if let Some(stack) = stacks.get_mut(&stack_num) {
    //         let mut reversed_stack = stack.clone();
    //         reversed_stack.reverse();
    //         stacks.insert(stack_num, reversed_stack);
    //     }
    //
    //     i += 1;
    // }
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
