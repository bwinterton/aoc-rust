use std::process;
use std::env;
use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file was passed in for instructions!");
        process::exit(1);
    }

    let mut f = File::open(&args[1]).unwrap();
    let mut instructions = String::new();
    f.read_to_string(&mut instructions).unwrap();

    let mut houses: HashSet<(i32,i32)> = HashSet::new();
    let mut robot_houses: HashSet<(i32,i32)> = HashSet::new();
    let mut current_house: (i32, i32) = (0,0);
    let mut santa_house: (i32, i32) = (0,0);
    let mut robot_house: (i32, i32) = (0,0);

    // Count the origin as receiving one present
    houses.insert(current_house);
    robot_houses.insert(robot_house);

    let mut ins_num = 1;
    let mut diff: (i32, i32) = (0,0);

    for c in instructions.chars() {
        // Figure out where we move to
        match c {
            '^' => {diff = (1, 0);},
            'v' => {diff = (-1, 0);},
            '>' => {diff = (0, 1);},
            '<' => {diff = (0, -1);},
            _ => {}
        }

        current_house = (current_house.0 + diff.0, current_house.1 + diff.1);
        houses.insert(current_house);

        if ins_num % 2 == 0 {
            santa_house = (santa_house.0 + diff.0, santa_house.1 + diff.1);
            robot_houses.insert(santa_house);
        } else {
            robot_house = (robot_house.0 + diff.0, robot_house.1 + diff.1);
            robot_houses.insert(robot_house);
        }

        ins_num += 1;
    }

    println!("Santa alone delivered presents to {} houses!", houses.len());
    println!("Santa with the help of Robot santa delivered presents to {} houses!!",
             robot_houses.len());
}

