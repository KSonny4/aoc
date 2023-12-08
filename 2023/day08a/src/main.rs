use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    let mut directions: String = "".to_string();
    let mut directions_se =false;

    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            if !directions_se &&  line.contains("L") && line.contains("R") {
                directions = line.trim().split_whitespace().collect();
                directions_se = true;
            } else if line == "" {
                // noop
            } else {
                let parts: Vec<&str> = line.trim().split("=").collect();
                let source: String = parts[0].trim().to_string();
                let dest: Vec<String> = parts[1]
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|c| c.trim().to_string())
                    .collect();
                mapping.insert(source, dest);
            }

            println!("Directions: {}", directions);
        });

    println!("{:?}", mapping);

    let mut location = "AAA".to_string();
    let mut index = 0;
    let direction_vec: Vec<String> = directions.chars().map(|c| c.to_string()).collect();
    let dir_len: usize = direction_vec.len();
    while location != "ZZZ".to_string() {
        let lr_directions = mapping.get(&location).unwrap();
        let m = direction_vec[index % dir_len].clone();
        location = match m.as_str() {
            "L" => lr_directions[0].clone(),
            "R" => lr_directions[1].clone(),
            _ => panic!("Should not happen"),
        };
        index += 1;
        //println!("{}", index);
    }

    println!("{}", index);
}
