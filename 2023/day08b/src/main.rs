use std::collections::HashMap;
use num;
fn main() {
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    let mut directions: String = "".to_string();
    let mut directions_se = false;

    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            if !directions_se && line.contains("L") && line.contains("R") {
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
           // println!("Directions: {}", directions);
        });

    println!("{:?}", mapping);


    let locations_starts: Vec<&String> = mapping.keys().filter(|x| x.ends_with("A")).collect();
    let mut index = 0;
    let direction_vec: Vec<char> = directions.chars().collect(); // Use char directly for efficiency
    let dir_len = direction_vec.len();

    let mut locations_z: Vec<i32> = Vec::new();
    for loc in locations_starts {
        let mut location = loc.clone();
        while !location.ends_with('Z') {
            let lr_directions = mapping.get(&location).unwrap();
            let m = direction_vec[index % dir_len].clone();
            location = match m {
                'L' => lr_directions[0].clone(),
                'R' => lr_directions[1].clone(),
                _ => panic!("Should not happen"),
            };
            index += 1;
            //println!("{}", index);
        }
        locations_z.push(index as i32);
        index = 0;
    }
    println!("{:?}", locations_z);
    let output: i64 = locations_z.iter().fold(1, |acc, &x| num::integer::lcm(x.into(), acc));

    println!("{}", output);
}
