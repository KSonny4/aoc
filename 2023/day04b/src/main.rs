fn main() {
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}
