// TODO: -ast-dump=json
//       tree_chain.binary_search()
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Inherits {
    public: Vec<String>,
    private: Vec<String>,
    protected: Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
struct Class {
    name: String,
    inherited: Inherits
}

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\| | *|\|*)*([|`])-[A-Z,a-z]*").unwrap();
        static ref WORD: Regex = Regex::new(r"([A-Z,a-z]+)").unwrap();
        static ref GET_CLASS: Regex = Regex::new(r"referenced class [_,A-Z,a-z,0-9]+ definition").unwrap();
    }

    // File hosts must exist in current path before this produces output
    if let Ok(read_lines) = read_lines("./src/dump.txt") {
        let mut tree_chain: Vec<String> = vec![];

        // Consumes the iterator, returns an (Optional) String
        for read_line in read_lines {
            if let Ok(ip) = read_line {
                let line = ip.as_str();

                if RE.is_match(line) {
                    let matched_string = RE.find(line).unwrap().as_str();
                    let keyword = WORD.find(matched_string).unwrap().as_str();
                    let level = (matched_string.len() - keyword.len()) / 2;

                    let chain_len = tree_chain.len();
                    if level == chain_len {
                        tree_chain.pop();
                    } else {
                        tree_chain.truncate(level - 1);
                    }
                    tree_chain.push(keyword.to_string());

                    if GET_CLASS.is_match(line) {
                        let matched = GET_CLASS.find(line).unwrap().as_str().split(" ");
                        let class_name = matched.collect::<Vec<&str>>()[2];

                        let class = r#"
                            {
                              "name": "class_name",
                              "inherited": {
                                "public": ["aa"],
                                "private": ["bb"],
                                "protected": ["cc"]
                              }
                            }
                        "#.replace("class_name", class_name);

                        let class: Class = serde_json::from_str(class.as_str()).unwrap();

                        println!("{:?}", class);
                    }
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