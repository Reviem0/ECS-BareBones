use core::panic;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

struct Interpreter {
    memory: HashMap<String, i32>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, lines: &[String], start: usize) -> usize {
        let mut i = start;
        while i < lines.len() {
            println!("{:?}", self.memory);
            let line = lines[i].trim();
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "clear" => {
                    self.memory.insert(tokens[1].trim_end_matches(';').to_owned(), 0);
                }
                "incr" => {
                    let counter = self.memory.entry(tokens[1].trim_end_matches(';').to_owned()).or_insert(0);
                    *counter += 1;
                }
                "decr" => {
                    let counter = self.memory.entry(tokens[1].trim_end_matches(';').to_owned()).or_insert(0);
                    *counter -= 1;
                }
                "while" => {
                    let mut loop_end = i + 1;
                    let mut depth = 1;
                    while depth > 0 && loop_end < lines.len() {
                        let loop_line = lines[loop_end].trim();
                        if loop_line.starts_with("while") {
                            depth += 1;
                        } else if loop_line.trim_end_matches(';') == "end" {
                            depth -= 1;
                        }
                        loop_end += 1;
                    }
                    if depth != 0 {
                        panic!("Unmatched while loop at line {}", i + 1);
                    }
                    while *self.memory.entry(tokens[1].to_owned()).or_insert(0) != 0 {
                        self.execute(lines, i + 1);
                    }
                    i = loop_end - 1; // Skip to the end of the loop
                }
                "end;" => {
                    return i; // Return from the current loop or function
                }
                _ => {
                    if !line.is_empty() {
                        panic!("Unknown command: {}", tokens[0]);
                    }
                }
            }
            i += 1;
        }
        i
    }
}

fn main() {
    let path = Path::new("Barebone.txt");
    if !path.exists() {
        panic!("Path does not exist");
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut interpreter = Interpreter::new();
    interpreter.execute(&lines, 0);
}