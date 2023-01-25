use std::io::Read;
use std::fs;
use std::path::Path;

fn main() -> Result<(), ()>{
    let mut memory: [u8; 1 << 16] = [0; 1 << 16];
    let mut mp: usize = 1 << 15;

    let args: Vec<String> = std::env::args().collect();
    if 1 == args.len() {
        // live interpreter
    } else {

        let mut i = 1; // skip the first argument (the program's name)
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!(
"Brainfuck Interpreter (cc Yqnk)

USAGE:
    rust-bf [OPTIONS]

OPTIONS:
    -t, --text <CODE>               Run <CODE> (Brainfuck code written between \"\" in the next argument)
    -f, --file <PATH_TO_FILE.b>     Run bf code in <PATH_TO_FILE.b>
    -h, --help                      Print help information"
                            );
                    i += args.len();
                }
                "-t" | "--text" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(())
                    } else {
                        run(args[i].to_owned(), &mut memory, &mut mp);
                    }
                }
                "-f" | "--file" => {
                    i += 1;
                    let path: &Path = Path::new(args[i].as_str());
                    if i >= args.len() {
                        panic!("1");
                    } else if path.exists() {
                        let content = fs::read_to_string(path).expect("error occured while reading file (see --help)");
                        run(content, &mut memory, &mut mp);
                    } else {
                        return Err(())
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
    Ok(())
}

fn run(source: String, memory: &mut [u8; 1 << 16], mp: &mut usize) {
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