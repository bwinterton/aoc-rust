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
    let final_floor = process_instructions(&instructions);

    let first_basement = find_first_basement(&instructions);

    println!("Santa ends up on floor {}", final_floor);
    println!("Entered basement for the first time at position {}",
             first_basement)
}

fn process_instructions(ins: &String) -> i32{
    let mut floor = 0;

    for c in ins.chars() {
        match c {
            '(' => { floor += 1; },
            ')' => { floor -= 1; },
            _ => {}
        }
    }
    floor
}

fn find_first_basement(ins: &String) -> i32 {
    let mut floor = 0;
    let mut pos = 1;

    for c in ins.chars() {
        match c {
            '(' => { floor += 1; },
            ')' => { floor -= 1; },
            _ => {}
        }

        if floor == -1 {
            return pos;
        }

        pos += 1;
    }

    pos
}

#[cfg(test)]
mod tests{
    use super::process_instructions;
    use super::find_first_basement;

    #[test]
    fn test_process() {
        assert_eq!(process_instructions(&String::from("((()))")), 0);
        assert_eq!(process_instructions(&String::from(")))(((")), 0);
        assert_eq!(process_instructions(&String::from("()()()")), 0);
        assert_eq!(process_instructions(&String::from("((())")), 1);
        assert_eq!(process_instructions(&String::from("(()))")), -1);
        assert_eq!(process_instructions(&String::from("(((((")), 5);
        assert_eq!(process_instructions(&String::from(")))))")), -5);
    }

    #[test]
    fn test_first_basement() {
        let cases: Vec<(String, i32)> = vec![
            (String::from(")"), 1),
            (String::from("(()))"), 5),
            (String::from("()())"), 5),
            (String::from("(()(())))"), 9),
            (String::from("())"), 3),
        ];

        for case in cases {
            assert_eq!(find_first_basement(&case.0), case.1);
        }
    }
}
