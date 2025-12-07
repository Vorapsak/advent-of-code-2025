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

fn part_two(input: &String) -> i64 {
    let mut count = 0;

    for interval in input.split(",") {
        let (left, right) = interval.split_once("-").unwrap();
        let left = str::parse::<i64>(left).unwrap();
        let right = str::parse::<i64>(right).unwrap();

        for i in left..=right {
            if is_invalid2(i) {
                count += 1;
                println!("found one: {}", i);
            }
        }
    }

    count
}

fn is_invalid2(i: i64) -> bool {
    let mut log10 = 1;
    while 10i64.pow(log10) < i {
        log10 += 1;
    }
    let log10 = log10 as i64;
    for length in 1i64..(log10/2 + 1) {
        if log10 % length == 0 {
            let mask = 10i64.pow(length.try_into().unwrap());
            let prefix = i / mask;
            let mut rest = i % mask;
            println!("prefix computed: {} for value {} with remainder {}", prefix, i, rest);
            while rest > prefix {
                if rest / prefix == 1 {
                    return true
                }
                rest %= prefix;
            }
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("example2.txt").unwrap();
    println!("{}", part_one(&input)); // 13108371860
    println!("{}", part_two(&input));
}