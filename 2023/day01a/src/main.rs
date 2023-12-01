fn main() {
    let res: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .filter(|ch| ch.is_digit(10))
                .collect::<String>()
        })
        .map(|line: String| {
            let first_char = line.chars().next();
            let last_char = line.chars().last();

            match (first_char, last_char) {
                (Some(first), Some(last)) => {
                    let result = format!("{}{}", first, last);
                    result
                }
                _ => String::from("0"),
            }
        })
        .collect();

    let result: i32 = res.into_iter().map(|s| s.parse::<i32>().unwrap_or(0)).sum();

    println!("{}", result)

    // for n in &res {
    //     println!("{}", n);
    // }
    //
}
