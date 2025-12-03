use std::fs;

fn part_one(input: &String) -> u32 {
    let mut total = 0;
    for line in input.split("\n") {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut d1 = 0;
        let mut i1 = 0;
        let mut d2 = 0;
        for (i, d) in digits.iter().enumerate() {
            if *d > d1 && i < digits.len()-1 {
                d1 = *d;
                i1 = i;
            }
        }
        for (i, d) in digits.iter().enumerate() {
            if *d > d2 && i > i1 {
                d2 = *d;
            }
        }
        total += d1*10 + d2;
    }

    total
}

fn part_two(input: &String) -> u64 {    
    let mut total: u64 = 0;
    for line in input.split("\n") {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    
        let mut acc: u64 = 0;
        let mut last_idx = -1isize;
        for i in (0..=11isize).rev() {
            acc *= 10;
            let mut d = 0;
            for idx in (last_idx+1)..(digits.len() as isize - i) {
                if digits[idx as usize] > d {
                    d = digits[idx as usize];
                    last_idx = idx;
                }
            }
            acc += d as u64;
        }
        total += acc;
    }


    total
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 17524
    println!("{}", part_two(&input)); // 173848577117276
}