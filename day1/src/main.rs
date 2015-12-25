use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file was passed in to be parsed");
        process::exit(1);
    }

    let mut f = File::open(&args[1]).unwrap();
    let mut instructions = String::new();
    f.read_to_string(&mut instructions).unwrap();
    let final_floor = process_instructions(instructions);

    println!("Santa ends up on floor {}", final_floor);
}

fn process_instructions(ins: String) -> i32{
    let mut floor = 0;

    for c in ins.chars() {
        match c {
            '(' => { floor += 1; },
            ')' => { floor -= 1; },
            _ => {}
        }
    }
    return return floor;
}
