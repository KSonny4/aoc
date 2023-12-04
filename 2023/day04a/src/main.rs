use std::collections::HashSet;

fn main() {
    let data: Vec<Vec<HashSet<String>>> = include_str!("input.txt")
        .lines()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap_or_default()
                .split('|')
                .map(|part| part.trim().split_whitespace().map(String::from).collect())
                .collect()
        })
        .collect();

    let scores: Vec<i32> = data.iter().map(|sets| match sets.len() {
        2 => sets[0].intersection(&sets[1]).count() as i32,
        _ => 0,
    }).collect();
    println!("{:?}", scores);

    let final_score: i32 = scores.iter().map(|&g| {
        match g {
            0 => 0,
            _ => 2_i32.pow((g - 1) as u32),
        }
    }).sum();
    println!("{:?}", final_score);
}
