use std::{fs};



fn part_one(input: &String) -> i64 {
    let mut count = 0;
    for interval in input.split(",") {
        let (left, right) = interval.split_once("-").unwrap();
        let left = str::parse::<i64>(left).unwrap();
        let right = str::parse::<i64>(right).unwrap();
        for i in left..=right {
            let mut log10 = 1;
            while 10i64.pow(log10) < i {
                log10 += 1;
            }
            if log10%2 == 1 {continue;}
            let midpoint = log10 / 2;
            let l = i / 10i64.pow(midpoint);
            let r = i % 10i64.pow(midpoint);
            if l == r {
                count += i;
            }
        }
    }

    count
}

fn part_two(_input: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 13108371860
    println!("{}", part_two(&input));
}