use lazy_static::lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    lazy_static! {
        // https://regex101.com/r/ehbOBe/1
        static ref RE: Regex = Regex::new(r"(\| | *|\|*)*([|`])-").unwrap();
    }

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./src/dump.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if RE.is_match(&ip) {
                    println!("{}", ip);
                } else {
                    println!("Nay")
                }
            };
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}