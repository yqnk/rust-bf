use std::io::Read;

fn main() {
    let mut memory: [u8; 1 << 16] = [0; 1 << 16];
    let mut mp: usize = 1 << 15;
    let source: String = String::from(",");
    run(source, &mut memory, &mut mp);
}

fn run(source: String, memory: &mut [u8; 1 << 16], mp: &mut usize) {
    let mut loop_stack = Vec::new();

    for (mut i, c) in source.chars().enumerate() {
        match c {
            '>' => *mp += 1,
            '<' => *mp -= 1,
            '+' => memory[*mp] += 1,
            '-' => memory[*mp] -= 1,
            '.' => print!("{}", memory[*mp] as char),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("error: cannot read stdin");
                memory[*mp] = input[0];
            },
            '[' => {
                if memory[*mp] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        i += 1;
                        if source.chars().nth(i) == Some('[') {
                            depth += 1;
                        } else if source.chars().nth(i) == Some(']') {
                            depth -= 1;
                        }
                    }
                } else {
                    loop_stack.push(i);
                }
            },
            ']' => {
                if memory[*mp] == 0 {
                    loop_stack.pop();
                } else {
                    let loop_start = loop_stack.last().cloned().expect("mismatched '[' and ']'");
                    i = loop_start;
                }
            },
            _ => {}
        }
    }
    if !loop_stack.is_empty() {
        panic!("end: mismatched '[' and ']'");
    }
}