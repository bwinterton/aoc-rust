extern crate crypto;

use std::env;
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Warning: No key was input! Please input a key to run!");
        process::exit(1);
    }

    let key = args[1];
    let md5 = Md5::new();

    for i in (0..std::u64::MAX) {
        md5.input(&key);
        md5.input(i);

        let out: String = md5.result_str();

        let first_five = i32::from_str(out[..5]);
    }
}
