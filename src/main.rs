use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

fn main() {
    // get user prog
    let path = env::args()
        .nth(1)
        .expect("pass *.bf as input");

    // parse into vector of chars for easier iteration
    let prog: Vec<char> = fs::read_to_string(path)
        .expect("error reading from file")
        .chars()
        .collect();

    // must be nonnegative
    let mut mem_ptr: usize = 0;
    let mut prog_ptr: usize = 0;

    let prog_len = prog.len();

    // memory for turing machi- I mean brainfuck interpreter
    let mut mem: Vec<u8> = vec![0];

    // stack for [] loops
    let mut loop_stack: Vec<usize> = Vec::new();

    while prog_ptr < prog_len {
        let cmd = prog.get(prog_ptr).expect("fatal prog ptr error");
        match cmd {
            '>' => {
                mem_ptr += 1;
                // grow vec if needed (this happens often at the beginning)
                if mem_ptr >= mem.len() {
                    mem.push(0);
                }
            },
            '<' => {
                // mem_ptr is a usize, but if this subtraction would overflow us
                // error out, since this is a user error in their code
                if mem_ptr == 0 {
                    eprintln!("fatal error, '<' encountered when tape pointer was at 0");
                    process::exit(1);
                }
                mem_ptr -= 1;
            },
            '+' => {
                mem[mem_ptr] += 1;
            },
            '-' => {
                mem[mem_ptr] -= 1;
            },
            '.' => {
                print!("{}", mem[mem_ptr] as char);
            },
            ',' => {
                let mut input = [0];
                io::stdin().read_exact(&mut input).expect("error reading user input");
                mem[mem_ptr] = input[0];
            },
            '[' => {
                // if loop is over
                if mem[mem_ptr] == 0 {
                    // look for next ']'
                    while prog[prog_ptr] != ']' {
                        prog_ptr += 1;
                        if prog_ptr >= prog_len {
                            eprintln!("no matching ']' for '['");
                            process::exit(1);
                        }
                    }
                } else {
                    loop_stack.push(prog_ptr);
                }
            },
            ']' => {
                // if current cell is nonzero, jump back to start of loop
                if mem[mem_ptr] != 0 {
                    prog_ptr = loop_stack.pop().unwrap_or_else(|| {
                        eprintln!("no matching '[' for ']'");
                        process::exit(1);
                    }) - 1; // minus one so we actually process the '[' again,
                            // adding it to the stack again
                } else {
                    // otherwise, we have a zero cell, and are done with loop,
                    // so remove prev bracket from stack
                    loop_stack.pop();
                }
            },
            '\n' => {
                process::exit(0);
            },
            _ => {
                eprintln!("unexpected character in bf program");
                process::exit(1);
            }
        }
        prog_ptr += 1;
    }
    println!();
}
