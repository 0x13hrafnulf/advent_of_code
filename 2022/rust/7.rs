use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../5.txt").unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
    let mut paths: HashMap<&str, u32> = HashMap::new();

    let mut path: Vec<&str> = Vec::new();

    for line in &lines {
        let parsed_line : Vec<&str> = line.split_whitespace().collect();

        if parsed_line[1] == "cd" {
            if parsed_line[2] == ".." {
                path.pop();
            }
            else {
                path.push(parsed_line[2]);
            }
        }
        else if parsed_line[1] == "ls" {
            continue;
        }
        if parsed_line[0] == "dir" {
            continue;
        }
        else {
            let size = parsed_line[0].parse::<u32>().unwrap();
            let n = path.len();

            for i in 0..n {
                paths
            }
        }

        
        
    }


}