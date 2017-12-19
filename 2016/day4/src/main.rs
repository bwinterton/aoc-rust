extern crate crypto;

use std::env;
use std::process;
use std::str::FromStr;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Warning: No key was input! Please input a key to run!");
        process::exit(1);
    }

    let key = args[1].clone();
    let mut md5 = Md5::new();

    for i in 0..std::u64::MAX {
        let temp_key: String = key + &i.to_string();
        md5.input(temp_key.as_bytes());

        let out: String = md5.result_str();

        let first_five = i32::from_str(&out[..5]);
    }
}
