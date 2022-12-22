use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("../5.txt").unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
    let mut stack : Vec<&str> = Vec::new();
    // Parse supplies
    let mut i  = 0;
    let size = lines.len();
    while i < size - 1 {
        stack.push(lines[i]);
        i += 1;

        if lines[i].len() <= 0 {
            break;
        }
    }

    let mut j = 1;
    let mut stacks : Vec<&str> = stack[i - j].split_whitespace().collect();
    let size = stacks.len();

    j += 1;
    while j <= i {
        let mut line = stack[i - j].replace(" ", "-");
        println!("{:?}", line);
        j += 1;   
    }

    // Process commands
}