use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn setup_rng(ln: &str) -> std::ops::Range<u8> {
    let nums: Vec<&str> = ln.split("-").collect();
    let lbound = nums[0].parse::<u8>().unwrap();
    let rbound = nums[1].parse::<u8>().unwrap();
    return std::ops::Range {start: lbound, end: rbound+1}
}

fn split_line(line: String) -> (HashSet<u8>, HashSet<u8>) {
    let line_split: Vec<&str> = line.split(",").collect();
    let lrng = HashSet::from_iter(setup_rng(line_split[0]));
    let rrng = HashSet::from_iter(setup_rng(line_split[1]));
    return (lrng, rrng)
}

fn main() {
    // Construct prority
    let fname = "/home/jvansan/projects/personal/advent-of-code-2022/day04/input";

    println!("Part 1:");
    let mut total_1 = 0;
    let mut total_2 = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            if let Ok(line_str) = line {
                let (lset, rset) = split_line(line_str);
                let intersect = &lset & &rset;
                if intersect == lset || intersect == rset {
                    // println!("{:?}", rset);
                    // println!("{:?}", lset);
                    total_1 += 1;
                }
                if intersect.len() > 0 {
                    total_2 += 1;
                }
            } 
        }
    } 

    println!("Total {:}", total_1);

    println!("Part 2:");
    println!("Total {:}", total_2);
}
