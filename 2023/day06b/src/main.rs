use std::iter::zip;
use std::time::Instant;

fn main() {

    // Too lazy to rewrite parsing :D
    let d:Vec<Vec<i64>> = vec![vec![35937366],vec![212206012011044]];

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
