use std::iter::zip;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let data: Vec<Vec<String>> = include_str!("input.txt")
        .lines()
        .map(|section| section.trim().split_whitespace().map(String::from).collect())
        .collect();

    let d: Vec<Vec<i32>> = data
        .iter()
        .map(|x| x[1..].to_vec())
        .map(|s| s.iter().map(|x| x.parse::<i32>().unwrap_or(0)).collect())
        .collect();


    let mut wins: Vec<i32> = Vec::new();
    for (time, distance) in zip(&d[0], &d[1]) {
        let mut win = 0;
        for t in 0..*time + 1 {
            let g = t * (*time - t);
            //println!("{t}: {}", g);
            if g > *distance {
                win += 1;
            }
        }
        wins.push(win);
    }

    let res = wins.iter().fold(1, |acc, dist| acc * dist);
    println!("{:?}", wins);
    println!("{:?}", res);
}
