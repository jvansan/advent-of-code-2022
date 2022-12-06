use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn do_problem(line_str: &str, size: usize) {
    let line_len = line_str.len();
    for i in 0..line_len {
        let mut end_idx = i + size;
        if end_idx >= line_len {
            end_idx = line_len - 1;
        }
        let slc = &line_str[i..end_idx];
        let set: HashSet<char>= HashSet::from_iter(slc.chars());
        if set.len() == size {
            println!("{}", i+size);
            break
        }
    }
}

fn main() {
    const TEST: bool = false;
    // Construct prority
    let fname: String;
    if TEST {
        fname = String::from("/home/jvansan/projects/personal/advent-of-code-2022/day06/test-input");
    } else {
        fname = String::from("/home/jvansan/projects/personal/advent-of-code-2022/day06/input");
    }

    println!("Part 1:");
    const SIZE: usize = 4;
    if let Ok(lines) = read_lines(&fname) {
        for line in lines{
            if let Ok(line_str) = line {
                do_problem(&line_str, SIZE);
            } 
        }
    } 

    println!("Part 2:");
    const SIZE2: usize = 14;
    if let Ok(lines) = read_lines(&fname) {
        for line in lines{
            if let Ok(line_str) = line {
                do_problem(&line_str, SIZE2);
            } 
        }
    } 
}
