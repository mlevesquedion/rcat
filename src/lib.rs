use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_args(args: &[String]) -> Result<&String, &'static str> {
    if args.len() < 2 {
        return Err("usage: rcat <filename>");
    }

    let filename = &args[1];
    Ok(filename)
}

#[test]
fn it_parses_args() {
    assert_eq!(
        parse_args(&vec!["progname".to_string(), "filename".into()]),
        Ok(&String::from("filename")),
    );
}

#[test]
fn it_fails_when_not_enough_args() {
    assert_eq!(
        parse_args(&vec!["progname".to_string()]),
        Err("usage: rcat <filename>")
    );
}
