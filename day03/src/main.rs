use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_line(line: String) -> (String, String) {
    let line_len = line.len()/2;
    let lstring = &line[..line_len];
    let rstring = &line[line_len..];
    return (lstring.to_string(), rstring.to_string())
}

fn main() {
    // Construct prority
    let mut item_prority = HashMap::new();
    let mut counter = 0;
    for c in b'a'..=b'z' {
        counter += 1;
        item_prority.insert(c as char, counter);
    }
    for c in b'A'..=b'Z' {
        counter += 1;
        item_prority.insert(c as char, counter);
    }
    let fname = "/home/jvansan/projects/advent-of-code-2022/day03/input";
    println!("Part 1:");
    let mut total = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            if let Ok(line_str) = line {
                let (lstring, rstring) = split_line(line_str);
                let lset: HashSet<char> = HashSet::from_iter(lstring.chars());
                let rset: HashSet<char> = HashSet::from_iter(rstring.chars());
                let matched = lset.intersection(&rset).nth(0).unwrap();
                total += item_prority.get(matched).unwrap();
            }
        }
    }
    println!("Total {:}", total);
    println!("Part 2:");
    let mut total_2 = 0;
    let mut stack: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            if let Ok(line_str) = line {
                stack.push(line_str);
                // This is an elf group
                if stack.len() == 3 {
                    let set_1: HashSet<char> = HashSet::from_iter(stack.pop().unwrap().chars());
                    let set_2: HashSet<char>  = HashSet::from_iter(stack.pop().unwrap().chars());
                    let set_3: HashSet<char>  = HashSet::from_iter(stack.pop().unwrap().chars());
                    let set_12: HashSet<char> = &set_1 & &set_2;
                    let matched = set_12.intersection(&set_3).nth(0).unwrap();
                    total_2 += item_prority.get(matched).unwrap();
                }
            }
        }
    }
    println!("Total {:}", total_2);
}
