// TODO: Could've used `clang -ast-dump=json`, but then what's the point
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Inherit {
    public: Vec<String>,
    private: Vec<String>,
    protected: Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Method {
    name: String,
    signature: String,
    used: bool
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Class {
    name: String,
    //TODO:
    // constructor: Option<String>,
    // destructor: Option<String>,
    inherited: Inherit,
    methods: Vec<Method>
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\| | *|\|*)*([|`])-[A-Z,a-z]*").unwrap();
    static ref WORD: Regex = Regex::new(r"([A-Z,a-z]+)").unwrap();
    static ref GET_CLASS: Regex = Regex::new(r"referenced class [_,A-Z,a-z,0-9]+ definition").unwrap();
    static ref GET_INHERITED_CLASS: Regex = Regex::new(r"'[_,A-Z,a-z,0-9]*'").unwrap();
    static ref GET_METHOD: Regex = Regex::new(r"(used )?[_,A-Z,a-z,0-9]+ '[(,),A-Z,a-z, ]+'").unwrap();
    static ref GET_METHOD_NAME: Regex = Regex::new(r"'[(,),A-Z,a-z, ]+'").unwrap();
}

pub struct State {
    class_list: Vec<Class>,
    tree_chain: Vec<String>,
    matched_string: String,
    keyword: String,
    level: usize
}

impl State {
    pub fn class_list(&self) -> &Vec<Class> {
        &self.class_list
    }
    pub fn tree_chain(&self) -> &Vec<String> {
        &self.tree_chain
    }
    fn initialize_state(&mut self, line: &String) {
        self.matched_string = RE.find(line).unwrap().as_str().to_owned();
        self.keyword = WORD.find(self.matched_string.as_str()).unwrap().as_str().to_owned();
        // Or instead of using a Regexp, I could do this:
        // self.keyword = self.matched_string.split_at(self.matched_string.find("-").unwrap() + 1).1.to_string();
        // Unsure which one is better (read: faster), perhaps I'll benchmark, I'm curious.

        // Note: Don't really need this and the logic after, there's no use of it in this project:
        //   It's there in case we need to know what our "branch" in the AST looks like and
        //   need to make decisions based on if our branch has some parent node or not.
        self.level = (self.matched_string.len() - self.keyword.len()) / 2;

        let chain_len = self.tree_chain.len();
        if self.level == chain_len {
            self.tree_chain.pop();
        } else {
            self.tree_chain.truncate(self.level - 1);
        }
        self.tree_chain.push(self.keyword.to_string());
    }

    pub fn process_line(&mut self, line: String) {
        self.initialize_state(&line);

        // CXXRecordDecl represents a C++ struct/union/class.
        if matches!(self.keyword.as_str(), "CXXRecordDecl") {
            // Since CXXRecordDecl represents struct/union/class, matching specifically for class decl:
            if GET_CLASS.is_match(&line) {
                let matched = GET_CLASS.find(&line).unwrap().as_str().split(" ");
                let class_name = matched.collect::<Vec<&str>>()[2];

                let class: Class = Class {
                    name: class_name.to_string(),
                    // constructor: None,
                    // destructor: None,
                    inherited: Inherit {
                        public: vec![],
                        private: vec![],
                        protected: vec![]
                    },
                    methods: vec![]
                };

                self.class_list.push(class);
            }
        }

        if matches!(self.keyword.as_str(), "CXXMethodDecl") {
            if GET_METHOD.is_match(&line) {
                let method_str = GET_METHOD.find(&line).unwrap().as_str();
                let method_signature = GET_METHOD_NAME.find(method_str).unwrap().as_str().replace("'", "");

                let method_data = method_str.split(" ").collect::<Vec<&str>>();
                // used add 'int (int, int)'
                let is_method_used = method_data.contains(&"used");
                let method: Method = Method {
                    name: if is_method_used {method_data[1].to_string()} else {method_data[0].to_string()},
                    signature: method_signature.to_string(),
                    used: is_method_used
                };

                let mut last = self.class_list.pop().unwrap();
                last.methods.push(method);
                self.class_list.push(last);
            }
        }

        // public, private and protected represent the classes that the current class inherits from
        if matches!(self.keyword.as_str(), "public" | "private" | "protected") {
            let inherited_from = GET_INHERITED_CLASS.find(&line).unwrap().as_str().replace("'", "");
            //TODO: Hacky, fix this, there's constant .pop() and .push()ing for no reason
            let mut last = self.class_list.pop().unwrap();

            match self.keyword.as_str() {
                "public" => {
                    last.inherited.public.push(inherited_from);
                }
                "private" => {
                    last.inherited.private.push(inherited_from);
                }
                "protected" => {
                    last.inherited.protected.push(inherited_from);
                }
                _ => {}
            }

            self.class_list.push(last);
        }
    }

    pub fn new() -> State {
        let class_list: Vec<Class> = vec![];
        let tree_chain: Vec<String> = vec![];
        let matched_string = String::new();
        let keyword = String::new();
        let level = 0;

        State {
            class_list,
            tree_chain,
            matched_string,
            keyword,
            level
        }
    }
}

fn main() {
    if let Ok(mut read_lines) = read_lines("./src/dump.txt") {
        // Skip the first line in the AST dump (TranslationUnitDecl)
        read_lines.next();

        let mut state = State::new();
        for read_line in read_lines {
            if let Ok(ip) = read_line {
                state.process_line(ip.as_str().to_owned());
            }
        }

        let json_output = json!({"program-data": state.class_list});
        println!("{}", json_output);
        if let Ok(mut file) = File::create("output.json") {
            file.write_all(json_output.to_string().as_ref()).unwrap();
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