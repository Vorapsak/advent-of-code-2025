use std::fs;

fn part_one(input: &String) -> u32 {
    let mut total = 0;
    for line in input.split("\n") {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut d1 = 0;
        let mut i1 = 0;
        let mut d2 = 0;
        let mut i2 = 0;
        for (i, d) in digits.iter().enumerate() {
            if *d > d1 && i < digits.len()-1 {
                d1 = *d;
                i1 = i;
            }
        }
        for (i, d) in digits.iter().enumerate() {
            if *d > d2 && i > i1 {
                d2 = *d;
                i2 = i;
            }
        }
        total += d1*10 + d2;
    }

    total
}

fn part_two(_: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 17524
    println!("{}", part_two(&input)); //
}