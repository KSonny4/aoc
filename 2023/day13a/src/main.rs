fn rotate_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return matrix.clone();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect()
}

fn is_subset(iter_: &Vec<String>, other: &Vec<String>) -> bool {
    for pair in other.iter().rev().cloned().zip(iter_) {
        if pair.0 != *pair.1 {
            return false;
        }
    }
    true
}


fn find_mirror(matrix: &Vec<Vec<char>>) -> Option<i32> {
    let rows = matrix.len();

    // Check for row mirrors
    for row_index in 1..rows {
        let first_half: Vec<String> = matrix[0..row_index]
            .iter()
            .map(|row| row.iter().collect())
            .collect();
        let second_half: Vec<String> = matrix[row_index..rows]
            .iter()
            .map(|row| row.iter().collect())
            .collect();

        if  is_subset(&second_half, &first_half) {
            return Some(row_index as i32);
        }
    }
    None
}

fn main() {
    let input: Vec<Vec<Vec<char>>> = include_str!("input.txt")
        .split("\n\n")
        .map(|section| section.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let result: Vec<i32> = input
        .iter()
        .flat_map(|table| {
            find_mirror(table).map(|row_index| row_index * 100)
        })
        .chain(input.iter().flat_map(|table| {
            find_mirror(&rotate_matrix(table)).into_iter()
        }))
        .collect();

    println!("{:?}", result);
    println!("{:?}", result.iter().sum::<i32>());
}
