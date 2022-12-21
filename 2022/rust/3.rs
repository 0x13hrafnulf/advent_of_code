use std::fs;

fn main() {
    let input = fs::read_to_string("../3.txt").unwrap();
    let lines : Vec<&str> = input.split("\n").collect();

    let mut score_part1 : i32 = 0;
    let mut score_part2 : i32 = 0;

    for line in lines {
        let (left, right) = line.trim().split_at(line.len() / 2);
        println!("1: {}", left);
        println!("2: {}", right);
        break;
    }
    
    println!("Score 1: {}", score_part1);
    println!("Score 2: {}", score_part2);
    
}