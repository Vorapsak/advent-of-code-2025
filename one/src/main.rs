use std::{cmp::max, fs};

use combine::parser::byte::num;

fn parseline(line: &str) -> i32 {
    let mul = match line.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        _ => panic!()
    };
    let val = str::parse::<i32>(&line[1..]).unwrap();
    mul * val
}

fn part_one(input: &String) -> i32 {
    let mut dial = 50;
    let mut num_zeroes = 0;

    for line in input.split("\n") {
        dial += parseline(line);
        dial %= 100;

        if dial == 0 {
            num_zeroes += 1;
        }
    }

    num_zeroes
}

fn part_two(input: &String) -> i32 {
    let mut dial = 50;
    let mut num_zeroes = 0;

    for line in input.split("\n") {
        let delta = parseline(line);
        num_zeroes += delta.abs() / 100;
        dial += delta % 100;
        
        if dial < 0 && (delta % 100).abs() > dial.abs() {
            num_zeroes += 1;
        } else if dial > 100 {
            num_zeroes += 1;
        }

        dial = dial.rem_euclid(100);
        if dial == 0 {
            num_zeroes += 1;
        }
    }

    num_zeroes
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 1052
    println!("{}", part_two(&input)); // 6295
}