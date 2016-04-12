use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::process;

struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file was passed in as input");
        process::exit(1);
    }

    let presents: Vec<Dimensions>  = parse_file(&args[1]);

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for present in presents {
        total_area += calculate_area(&present);
        total_ribbon += calculate_ribbon_length(&present);
    }

    println!("Total amount of wrapping paper needed: {} sqft", total_area);
    println!("Total ribbon needed: {} ft", total_ribbon);
}

fn calculate_area(dim: &Dimensions) -> i32{
    // Calculate the areas for the 3 sets of sides
    let area_1 = dim.length * dim.width;
    let area_2 = dim.width * dim.height;
    let area_3 = dim.height * dim.length;

    // Find the smallest area for the slack
    let mut smallest_area = area_1;
    if area_2 < smallest_area {
        smallest_area = area_2;
    }
    if area_3 < smallest_area {
        smallest_area = area_3;
    }

    // Calculate total square feet of wrapping paper needed.
    ((2 * area_1) +
     (2 * area_2) +
     (2 * area_3) + smallest_area)
}

fn calculate_ribbon_length(dim: &Dimensions) -> i32 {
    let mut perims: Vec<i32> = Vec::new();

    perims.push((dim.length * 2) + (dim.width * 2));
    perims.push((dim.width * 2) + (dim.height * 2));
    perims.push((dim.height * 2) + (dim.length * 2));

    // Sort to get the smallest
    perims.sort();

    // Necessary ribbon is the smallest perimeter + the volume
    perims[0] + (dim.length * dim.width * dim.height)
}

fn parse_file(filename: &String) -> Vec<Dimensions> {

    let f = File::open(filename).unwrap();
    let r = BufReader::new(f);

    let mut dims: Vec<Dimensions> = Vec::new();

    // For each line split and then push into the dimension object
    for l in r.lines() {
        let s = l.unwrap();
        let input: Vec<&str> = s.split("x").collect();
        dims.push(Dimensions {length: input[0].parse::<i32>().unwrap(),
                              width: input[1].parse::<i32>().unwrap(),
                              height: input[2].parse::<i32>().unwrap()});
    }

    dims

}
