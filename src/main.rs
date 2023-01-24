use std::io::Read;

fn main() {
    //let source = ",[ .,]"; // cat
    let source = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."; // hello world
    let mut memory: [u8; 1 << 18] = [0; 1 << 18];
    let mut mp: usize = 1 << 17;
    run(source.to_string(), &mut memory, &mut mp);
}

fn run(source: String, memory: &mut [u8; 1 << 18], mp: &mut usize) {
    let mut indexes: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    while index < source.len() {
        let (_i, c) = source.chars().enumerate().nth(index).unwrap();
        match c {
            '>' => *mp += 1,
            '<' => *mp -= 1,
            '+' => memory[*mp] = memory[*mp].checked_add(1).unwrap_or(0),
            '-' => memory[*mp] = memory[*mp].checked_sub(1).unwrap_or(255),
            '.' => print!("{}", memory[*mp] as char),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("error: cannot read stdin");
                memory[*mp] = input[0];
            },
            '[' => {
                indexes.push(index);
            },
            ']' => {
                if memory[*mp] != 0 {
                    index = indexes.pop().unwrap() - 1;
                } else {
                    indexes.pop();
                }
            },
            _ => {}
        }
        index += 1;
    }

    if !indexes.is_empty() {
        dbg!(&indexes);
        panic!("end: mismatched '[' and ']'");
    }
}