extern crate rcat;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = rcat::parse_args(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let contents = rcat::read_file(&filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("{}", contents);
}
