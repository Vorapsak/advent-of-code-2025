use std::fs;

fn part_one(input: &String) -> u32 {
    let mut count = 0;
    let input: Vec<&str> = input.split("\n").collect();
    let height = input.len();
    let width = input[0].len();
    
    


    count
}

fn part_two(_: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("example.txt").unwrap();
    println!("{}", part_one(&input)); // 
    println!("{}", part_two(&input)); // 
}