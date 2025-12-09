use std::fs;



fn part_one(input: &String) -> u64 {
    let mut max_size = 0;
    let mut points: Vec<(u64, u64)> = vec![];
    for line in input.split("\n") {
        if let Some((s1, s2)) = line.split_once(",") {
            let current_point = (str::parse::<u64>(s1).unwrap(), str::parse::<u64>(s2).unwrap());
            if points.len() > 0 {
                for cmp in points.iter() {
                    let size = ((current_point.0).abs_diff(cmp.0)+1) * ((current_point.1).abs_diff(cmp.1)+1);
                    if size > max_size {
                        //println!("new max size: {size} between {:?} and {:?}", current_point, cmp);
                        max_size = size;
                    }
                }
            }
            points.push(current_point);
        }
    }

    max_size
}

fn part_two(_: &String) -> &'static str {
    "incomplete"
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_one(&input)); // 4715966250
    println!("{}", part_two(&input)); // 
}