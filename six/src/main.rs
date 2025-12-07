use std::fs;


fn part_one(input: &String) -> u64 {
    let mut sum = 0;
    let rows: Vec<_> = input.split("\n").collect();
    let mut new_rows: Vec<Vec<u64>> = Vec::new();
    let mut ops = Vec::new();
    for row in rows {
        if row.chars().nth(0) == Some('*') || row.chars().nth(0) == Some('+') {
            ops = row.split(" ").filter(|x| !x.is_empty()).collect();
        } else {
            new_rows.push(row.split(" ").filter_map(|x| 
                if !x.is_empty() {
                    Some(str::parse::<u64>(x).unwrap())
                } else {
                    None
                }
            ).collect::<Vec<_>>());
        }
    }
    
    for i in 0..ops.len() {
        let mut acc = 0;
        match ops[i] {
            "*" => {
                acc = 1;
                for row in new_rows.iter() {
                    acc *= row[i];
                }
            }
            "+" => {
                acc = 0;
                for row in new_rows.iter() {
                    acc += row[i];
                }
            }
            _ => panic!(),
        }
        sum += acc;
    }

    sum
}

fn part_two(_input: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 6295830249262
    println!("{}", part_two(&input)); // 
}