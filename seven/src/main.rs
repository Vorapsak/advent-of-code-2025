use std::fs;

fn part_one(input: &String) -> i32 {
    let mut splits = 0;
    let rows = input.split("\n").into_iter();
    let row1 = rows.next().unwrap();



    splits
}

fn part_two(_: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("example.txt").unwrap();
    println!("{}", part_one(&input)); // 
    println!("{}", part_two(&input)); // 
}