use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
struct Stack {
    items: Vec<Vec<char>>
}

impl Stack {
    fn apply_9000(&mut self, inst: &Instruction) {
        let (id_from, id_to) = instruction_to_indices(inst);
        for _ in 0..inst.count {
            let val = self.items[id_from].pop().unwrap();
            self.items[id_to].push(val);
        } 
    }

    fn apply_9001(&mut self, inst: &Instruction) {
        let (id_from, id_to) = instruction_to_indices(inst);
        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..inst.count {
            let val = self.items[id_from].pop().unwrap();
            temp_stack.push(val);
        } 
        for val in temp_stack.iter().rev() {
            self.items[id_to].push(*val);
        }
    }

    fn result(&self) -> String {
        let mut res: String = "".to_string();
        for i in 0..self.items.len() {
            let ilen = self.items[i].len();
            res.push_str(&self.items[i][ilen-1].to_string()); 
        }
        return res
    }
}

fn build_stack(test: bool) -> Stack {
    if test {
        return Stack {items: vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ]}
    } else {
        return Stack {items: vec![
            vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'],
            vec!['D', 'Q', 'L'],
            vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'],
            vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'],
            vec!['V', 'G', 'L', 'F', 'Z', 'S'],
            vec!['D', 'G', 'N', 'P'],
            vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'],
            vec!['C', 'P', 'D', 'M', 'S'],
            vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'],
        ]}
    }
}


#[derive(Debug)]
struct Instruction {
    count: u8,
    from: String,
    to: String,
}

fn build_instruction(line: String) -> Instruction {
    let line_split: Vec<String> = line.split(" ").map(|x| String::from(x)).collect();
    let count = line_split[1].parse::<u8>().unwrap();
    return Instruction { count, from: line_split[3].clone(), to: line_split[5].clone() }
}

fn instruction_to_indices(inst: &Instruction) -> (usize, usize) {
    let id_from = inst.from.parse::<usize>().unwrap() - 1;
    let id_to = inst.to.parse::<usize>().unwrap() - 1;
    return (id_from, id_to)
}

fn main() {
    const TEST: bool = false;
    // Construct prority
    let fname: String;
    if TEST {
        fname = String::from("/home/jvansan/projects/personal/advent-of-code-2022/day05/test-input");
    } else {
        fname = String::from("/home/jvansan/projects/personal/advent-of-code-2022/day05/input");
    }

    let mut stacks = build_stack(TEST);
    let mut instructions: Vec<Instruction> = Vec::new();
    println!("Starting stacks: {:?}", stacks.items);
    println!("Part 1:");
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            if let Ok(line_str) = line {
                let inst = build_instruction(line_str);
                println!("{:?}", inst);
                let _ = &stacks.apply_9000(&inst);
                instructions.push(inst);
            } 
        }
    } 
    println!("Final stacks: {:?}", stacks.items);
    println!("{}", stacks.result());

    // println!("Total {:}", total_2);
    let mut stacks = build_stack(TEST);
    println!("Part 2:");
    for inst in instructions.iter() {
        println!("{:?}", inst);
        let _ = &stacks.apply_9001(&inst);
    }
    println!("Final stacks: {:?}", stacks.items);
    println!("{}", stacks.result());
}
