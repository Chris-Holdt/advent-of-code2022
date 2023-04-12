use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

#[derive(Debug)]
struct FileTree {
    name: String,
    is_dir: bool,
    size: i32,
    sub_tree: HashMap<String, FileTree>,
    files: Vec<FileTree>,
}

type Lines = io::Lines<io::BufReader<File>>;

fn main() {
    let lines = get_puzzle_input("./input.test");
    // let lines = get_puzzle_input("./input.main");

    let mut file_tree: HashMap<String, FileTree> = HashMap::new();
}

fn build_tree(tree: &mut HashMap<String, FileTree>, lines: Lines) {
    let cmd_start = String::from("$");
    let mut i = 0;

    for line_result in lines {
        let line: String;

        match line_result {
            Ok(l) => line = l.clone(),
            Err(e) => {
                dbg!(e);
                panic!()
            }
        }

        let line_split: Vec<String> = line.split_whitespace().map(|l| String::from(l)).collect();

        if line_split.contains(&cmd_start) {
            if line_split[1] == String::from("cd") {
                // handle change dir

                continue;
            }

            if line_split[1] == String::from("ls") {
                // handle change dir

                let mut got_cmd = false;
                let listing: Vec<String> = vec![];

                while got_cmd != true {
                    let next_line: Result<String, std::io::Error>;

                    match lines.next() {
                        Some(l) => next_line = l,
                        None => return,
                    }
                }
                continue;
            }
        }
    }
}

fn sample_input() -> String {
    String::from("")
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
