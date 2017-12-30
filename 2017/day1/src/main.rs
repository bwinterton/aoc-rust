extern crate clap;
use clap::{Arg, App};

fn main() {
    let args = App::new("Rusty AoC Day 1")
        .version("1.0")
        .author("Brayden Winterton <bwinterton@gmail.com>")
        .about("Solves Day 1 of the Advent of Code")
        .arg(Arg::with_name("input")
            .required(true)
            .index(1)
            .help("The puzzle input"))
        .get_matches();
    
    let input = args.value_of("input").unwrap();
    let answer = solve(input);

    println!("The solution to the first half is: {}", answer.0);
    println!("The solution to the second half is: {}", answer.1);
}

fn solve(input: &str) -> (i32, i32) {
    let mut answer1: i32 = 0;
    let mut answer2: i32 = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut c: char;
    let half = chars.len()/2;

    for i in 1..chars.len() {
        c = chars[i];

        // Solve for first half
        if c == chars[(i+1) % chars.len()]{
            answer1 += c.to_digit(10).unwrap() as i32;
        }

        // Solve for second half
        if c == chars[(i+half) % chars.len()] {
            answer2 += c.to_digit(10).unwrap() as i32;
        }
    }


    return (answer1, answer2);

}
