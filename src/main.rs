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
use std::io::{self, Write};
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

fn print_debug_state(memory: &[u8], memory_pointer: usize, instruction_pointer: usize, instruction: char) {
    // Print debug state to the console
    println!("Memory[{}]: {}", memory_pointer, memory[memory_pointer]);
    println!("Memory Pointer: {}", memory_pointer);
    println!("Instruction Pointer: {}", instruction_pointer);
    println!("Instruction: {}", instruction);
}

fn run(code: &[char], debug: bool, step: bool) {

    // Initialize variables
    const MEMORY_SIZE: usize = 30000;
    let mut instruction_pointer: usize = 0;
    let mut memory_pointer: usize = 0;
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];

    // Run the program
    while instruction_pointer < code.len() {

        // initialize instruction
        let instruction = code[instruction_pointer];

        // Check for debugging mode
        if debug {
            print_debug_state(&memory, memory_pointer, instruction_pointer, instruction);
        }

        // Check for step mode
        if step {
            let mut input = String::new();
            print!("Press enter to step... ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
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
                let mut buffer = [0u8; 1];
                if io::stdin().read_exact(&mut buffer).is_err() {
                    buffer[0] = 0;
                }
                memory[memory_pointer] = buffer[0];
            }
            '[' => {
                if memory[memory_pointer] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        instruction_pointer += 1;
                        if instruction_pointer >= code.len() {
                            eprintln!("Error: Unmatched '[' at index {}", instruction_pointer);
                            return;
                        }
                        match code[instruction_pointer] {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[memory_pointer] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if instruction_pointer == 0 {
                            eprintln!("Error: Unmatched ']' at index 0");
                            return;
                        }
                        instruction_pointer -= 1;
                        match code[instruction_pointer] {
                            '[' => depth -= 1,
                            ']' => depth += 1,
                            _ => {}
                        }
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
    
    // convert to chars
    let code: Vec<char> = contents.chars().collect();

    // Run the program
    run(&code, args.debug, args.step);
}
