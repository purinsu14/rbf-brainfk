// BRAINFUCK INTERPRETER IN RUST
//
// This is a brainfuck interpreter written in Rust. It takes a .bf file as input and
// outputs the result of the program to the console.
// 
// To run the program, use the following command:
//     cargo run -- <input_file.bf>
//

// Import dependencies
use clap::Parser;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::fs;

// Define the command-line arguments
/// BRAINFUCK INTERPRETER IN RUST
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file path
    file: std::path::PathBuf,

    /// Enable debugging mode
    #[arg(short, long)]
    debug: bool,

    /// Enable stepping mode (press enter to step through the code)
    #[arg(short, long)]
    step: bool,
}

fn print_debug_state(memory: &[u8], mp: usize, ip: usize, instruction: char) {
    // Print the current state of the program
    println!("[IP:{:4}] [MP:{:4}] [VAL:{:3}] INST: {}", ip, mp, memory[mp], instruction);
}

fn run(code: &[char], debug: bool, step: bool) {

    // Initialize variables
    const MEMORY_SIZE: usize = 30000;
    let mut instruction_pointer: usize = 0;
    let mut memory_pointer: usize = 0;
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];

    // initialize jump table and bracket stack
    let mut jump_table = HashMap::new();
    let mut bracket_stack = Vec::new();

    // map brackets to jump table
    for (i, &c) in code.iter().enumerate() {
        if c == '[' {
            bracket_stack.push(i);
        } else if c == ']' {
            if let Some(j) = bracket_stack.pop() {
                jump_table.insert(j, i);
                jump_table.insert(i, j);
            } else {
                eprintln!("Error: Unmatched ']' at index {}", i);
                return;
            }
        }
    }
    // check for unmatched brackets
    if !bracket_stack.is_empty() {
        eprintln!("Error: Unmatched '[' at index {}", bracket_stack.last().unwrap());
        return;
    }

    // Run the program
    while instruction_pointer < code.len() {

        // initialize instruction
        let instruction = code[instruction_pointer];

        // Check for debugging mode
        if debug {
            print_debug_state(&memory, memory_pointer, instruction_pointer, instruction);
            // check for step mode
            if step {
                print!("Press enter to step... ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }
        }

        // instructions
        match instruction {
            '>' => memory_pointer = (memory_pointer + 1) % MEMORY_SIZE,
            '<' => memory_pointer = (memory_pointer + MEMORY_SIZE - 1) % MEMORY_SIZE,
            '+' => memory[memory_pointer] = memory[memory_pointer].wrapping_add(1),
            '-' => memory[memory_pointer] = memory[memory_pointer].wrapping_sub(1),
            '.' => {
                print!("{}", memory[memory_pointer] as char);
                io::stdout().flush().unwrap();
            }
            ',' => {
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_ok() {
                    memory[memory_pointer] = input.bytes().next().unwrap_or(0);
                } else {
                    memory[memory_pointer] = 0;
                }
            }
            '[' => {
                if memory[memory_pointer] == 0 {
                    if let Some(&target) = jump_table.get(&instruction_pointer) {
                        instruction_pointer = target;
                    }
                }
            }
            ']' => {
                if memory[memory_pointer] != 0 {
                    if let Some(&target) = jump_table.get(&instruction_pointer) {
                        instruction_pointer = target;
                    }
                }
            }
            _ => {}
        }
        instruction_pointer += 1;
    }
}

fn main() {
    // Capture args
    let args = Args::parse();

    // Read contents of input file
    let contents = fs::read_to_string(&args.file).expect("Failed to read file");
    
    // convert to chars and filter other characters
    let code: Vec<char> = contents.chars().filter(|&c| "<>+-.,[]".contains(c)).collect();

    // Run the program
    run(&code, args.debug, args.step);
}
