extern crate clap;
use clap::{App, Arg};
use std::process;
use std::error;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args = App::new("Rusty AoC Day 2")
        .version("1.0")
        .author("Brayden Winterton <bwinterton@gmail.com>")
        .about("Produces the solution for Day 2 of the AoC 2017")
        .arg(Arg::with_name("input_file")
            .index(1)
            .required(true)
            .help("The spreadsheet to input"))
        .get_matches();
    
    let input_file = args.value_of("input_file").unwrap();
    let solutions = match solve(input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}" ,e);
            process::exit(1);
        },
    };
    println!("Solution for the first half: {}", solutions.0);
    println!("Solution for the second half: {}", solutions.1);
}

fn solve(input_file: &str) -> Result<(i32, i32), Box<error::Error>> {
    let (mut answer1, mut answer2) = (0, 0);

    let values = parse_input(input_file)?;

    // Iterate through rows
    for y in 0..values.len() {
        let (mut min, mut max) = (10000, 0);

        // columns
        for x in 0..values[y].len() {
            if values[y][x] < min {
                min = values[y][x];
            }
            if values[y][x] > max {
                max = values[y][x];
            }

            // Solve for second half
            for z in 0..values[y].len() {
                if (values[y][z] % values[y][x]) == 0 &&
                    values[y][z] != values[y][x] {
                    answer2 += values[y][z] / values[y][x];
                }
            }
        }
        answer1 += max - min;
    }

    Ok((answer1, answer2))
}

fn parse_input(input_file: &str) -> Result<Vec<Vec<i32>>, Box<error::Error>> {

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut values: Vec<Vec<i32>> = Vec::new();

    let mut i = 0;
    for line in reader.lines() {
        values.push(Vec::new());
        for val in line.unwrap().split('\t') {
            values[i].push(val.parse()?);
        }
        i += 1;
    }

    Ok(values)

}