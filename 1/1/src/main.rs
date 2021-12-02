use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    println!("Hello World!");
    let input_path = ::std::env::args().nth(1).unwrap();
    let handle = File::open(&input_path);
    match handle {
        Err(ref e) => {
            println!("failed to open file: {:?}", e);
            return
        },
        Ok(ref v) => println!("opened it! {:?}", v),
    }
    let unwrapped = handle.unwrap();
    let file = BufReader::new(unwrapped);
    for line in file.lines() {
        let my_line = line.unwrap();
        println!("{}", my_line);
    }
}

