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
    inherited: Inherit,
    methods: Vec<Method>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Struct {
    name: String,
    variables: Vec<Variable>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Variable {
    name: String,
    variable_type: String
    // TODO: Can have used/referenced tag in a Variable as well
}

lazy_static! {
    // TODO:
    //    merge these and clean up the `if`s in State.process_line()
    static ref RE: Regex = Regex::new(r"(\| | *|\|*)*([|`])-[A-Za-z]*").unwrap();
    static ref WORD: Regex = Regex::new(r"([A-Za-z]+)").unwrap();
    // TODO: referenced for Class: (referenced )?[_,A-Z,a-z,0-9]+ '[(,),A-Z,a-z, ,*]+'
    static ref GET_CLASS: Regex = Regex::new(r"class [\w]+ definition").unwrap();
    static ref GET_STRUCT: Regex = Regex::new(r"struct [\w]+ definition").unwrap();
    static ref GET_INHERITED_CLASS: Regex = Regex::new(r"'[\w]*'").unwrap();
    static ref GET_METHOD: Regex = Regex::new(r"(used )?[\w]+ '[()A-Za-z ,*]+'").unwrap();
    static ref GET_METHOD_NAME: Regex = Regex::new(r"'[()A-Za-z ,*]+'").unwrap();
    // TODO: used/referenced for Variable: (used |referenced )?[_,A-Z,a-z,0-9]+ '[(,),A-Z,a-z, ,*]+'
    static ref GET_VARIABLE: Regex = Regex::new(r"[\w]+ '[()A-Za-z ,*]+'").unwrap();
}

pub struct State {
    struct_list: Vec<Struct>,
    class_list: Vec<Class>,
    tree_path: Vec<String>,
    matched_string: String,
    keyword: String,
    level: usize
}

impl State {
    fn search_tree_path(&mut self, keyword: &str) -> bool {
        for line in self.tree_path.clone().into_iter().rev() {
            if line.contains(keyword) {
                return true;
            }
        }
        return false;
    }
    fn initialize_state(&mut self, line: &String) {
        self.matched_string = RE.find(line).unwrap().as_str().to_owned();
        // TODO: instead of a Regex:
        //  Split the matched_string at '-' and get the keyword that way:
        //  Unsure which one is better(/faster), perhaps I'll benchmark, (nitpick but I'm curious).
        self.keyword = WORD.find(self.matched_string.as_str()).unwrap().as_str().to_owned();
        self.level = (self.matched_string.len() - self.keyword.len()) / 2;

        // Tracking our current branch/path in the AST
        let chain_len = self.tree_path.len();
        if self.level == chain_len {
            self.tree_path.pop();
        } else {
            self.tree_path.truncate(self.level - 1);
        }
        self.tree_path.push(line.to_string());
    }

    pub fn process_line(&mut self, line: String) {
        self.initialize_state(&line);

        match self.keyword.as_str() {
            // CXXRecordDecl matches a C++ struct/union/class, processing it here.
            "CXXRecordDecl" => { self.process_record_decl(&line); }
            // Processes variable declarations
            "FieldDecl" => { self.process_field_decl(&line); }
            // Processing the methods of a class.
            "CXXMethodDecl" => { self.process_method_decl(&line); }
            // public/private/protected represent the classes that the current class inherits from.
            "public" | "private" | "protected" => { self.process_class_inherits(&line); }
            _ => {}
        }
    }

    // Processes C++ struct/union/class.
    fn process_record_decl(&mut self, line: &String) {
        // Matching specifically for class decl:
        if GET_CLASS.is_match(&line) {
            let matched = GET_CLASS.find(&line).unwrap().as_str().split(" ");
            let class_name = matched.collect::<Vec<&str>>()[1];
            let class: Class = Class {
                name: class_name.to_string(),
                inherited: Inherit {
                    public: vec![],
                    private: vec![],
                    protected: vec![]
                },
                methods: vec![]
            };
            self.class_list.push(class);
        }

        // A union would be more of the same.
        // Matching specifically for struct decl:
        if GET_STRUCT.is_match(&line) {
            let matched = GET_STRUCT.find(&line).unwrap().as_str().split(" ");
            let struct_name = matched.collect::<Vec<&str>>()[1];
            let new_struct = Struct {
                name: struct_name.to_string(),
                variables: vec![]
            };
            self.struct_list.push(new_struct);
        }
    }

    // Processes any CXXFieldDecl.
    fn process_field_decl(&mut self, line: &String) {
        if GET_VARIABLE.is_match(&line) {
            let variable_decl = GET_VARIABLE.find(&line).unwrap().as_str();
            let variable_data = variable_decl.split(" ").collect::<Vec<&str>>();
            let variable_name = variable_data[0].to_string();
            let variable_signature = variable_data[1].replace("'", "");

            let variable: Variable = Variable {
                name: variable_name,
                variable_type: variable_signature,
            };

            if self.search_tree_path("struct") {
                let mut last_struct = self.struct_list.pop().unwrap();
                last_struct.variables.push(variable);
                self.struct_list.push(last_struct);
            }
        }
    }

    // Processes any class methods.
    fn process_method_decl(&mut self, line: &String) {
        if GET_METHOD.is_match(&line) {
            let method_str = GET_METHOD.find(&line).unwrap().as_str();
            let method_signature = GET_METHOD_NAME.find(method_str).unwrap().as_str().replace("'", "");
            let method_data = method_str.split(" ").collect::<Vec<&str>>();
            let is_method_used = method_data.contains(&"used");

            let method: Method = Method {
                name: if is_method_used {method_data[1].to_string()} else {method_data[0].to_string()},
                signature: method_signature.to_string(),
                used: is_method_used
            };

            let mut last_class = self.class_list.pop().unwrap();
            last_class.methods.push(method);
            self.class_list.push(last_class);
        }
    }

    // Processes the public, private and protected class inheritances of a class.
    fn process_class_inherits(&mut self, line: &String) {
        let inherited_from = GET_INHERITED_CLASS.find(&line).unwrap().as_str().replace("'", "");

        let mut last = self.class_list.pop().unwrap();
        match self.keyword.as_str() {
            "public" => { last.inherited.public.push(inherited_from) }
            "private" => { last.inherited.private.push(inherited_from) }
            "protected" => { last.inherited.protected.push(inherited_from) }
            _ => {}
        }
        self.class_list.push(last);
    }

    pub fn new() -> State {
        State {
            struct_list: vec![],
            class_list: vec![],
            tree_path: vec![],
            matched_string: String::new(),
            keyword: String::new(),
            level: 0
        }
    }
}

fn main() {
    if let Ok(mut read_lines) = read_lines("./src/dump.txt") {
        // Skip the first line in the AST dump (TranslationUnitDecl)
        read_lines.next();

        // Then process the rest of the AST
        let mut state = State::new();
        for read_line in read_lines {
            if let Ok(ip) = read_line {
                state.process_line(ip.as_str().to_owned());
            }
        }

        // Print the output and write it to output.json
        let json_output = json!({"class-data": state.class_list, "struct-data": state.struct_list});
        let json_prettified = serde_json::to_string_pretty(&json_output).unwrap();
        println!("{}", json_prettified);
        if let Ok(mut file) = File::create("output.json") {
            file.write_all(json_prettified.to_string().as_ref()).unwrap();
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