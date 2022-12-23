use std::fs;
use std::collections::VecDeque;

fn main() {
    let input = fs::read_to_string("../6.txt").unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
  
    let deque: VecDeque<char> = VecDeque::new();

    
}