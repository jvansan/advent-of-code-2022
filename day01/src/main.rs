use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let fname = "/home/jvansan/projects/advent-of-code-2022/day01/input";
    let mut count_vec = Vec::new();
    let mut value = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            if let Ok(count) = line {
                if count.len() == 0 {
                    count_vec.push(value);
                    value = 0;
                    continue;
                }
                let line_value = count.parse::<i32>().unwrap();
                value += line_value;
            }
        }
    }
    // println!("{:?}", count_vec);
    count_vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}", count_vec[0]);
    println!("{:?}", &count_vec[0..3]);
    println!("{:?}", &count_vec[0..3].iter().sum::<i32>());
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
