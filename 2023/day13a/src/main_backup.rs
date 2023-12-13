use std::os::unix::process::parent_id;
// fn rotate_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
//     if matrix.is_empty() || matrix[0].is_empty() {
//         return matrix.clone();
//     }
//
//     let rows = matrix.len();
//     let cols = matrix[0].len();
//
//     (0..cols)
//         .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
//         .collect()
// }
//
// fn is_subset(iter_: &Vec<String>, other: &Vec<String>) -> bool {
//     other.iter().rev().cloned().all(|x_other| iter_.contains(&x_other))
// }
//
// fn find_mirror(matrix: &Vec<Vec<char>>, type_: &str) -> Option<(usize, String)> {
//     let rows = matrix.len();
//
//     // Check for row mirrors
//     for row_index in 1..rows {
//         let first_half: Vec<String> = matrix[0..row_index]
//             .iter()
//             .map(|row| row.iter().collect())
//             .collect();
//         let second_half: Vec<String> = matrix[row_index..rows]
//             .iter()
//             .map(|row| row.iter().collect())
//             .collect();
//
//         if is_subset(&first_half, &second_half) || is_subset(&second_half, &first_half) {
//             return Some((row_index, type_.to_string()));
//         }
//     }
//
//     // Do something with rotated_matrix
//
//     None
// }
//
// fn main() {
//     let input: Vec<Vec<char>> = include_str!("input_simple1.txt")
//         .lines()
//         .map(|line| line.chars().collect())
//         .collect();
//
//     if let Some((index, kind)) = find_mirror(&input, "row") {
//         println!("Mirror found at {}th {}.", index, kind);
//     } else {
//         println!("No mirror found.");
//     }
//
//     let rotated_matrix = rotate_matrix(&input);
//     if let Some((index, kind)) = find_mirror(&rotated_matrix, "col") {
//         println!("Mirror found at {}th {}.", index, kind);
//     } else {
//         println!("No mirror found.");
//     }
// }


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
    // vzit kratsi a proovnat na stejnych indexech, contains nefunguje..

    for pair in  other.iter().rev().cloned().zip(other) {
        if pair.0 != pair.1 {
            return false
        }
    }
    return true
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

        if is_subset(&first_half, &second_half) || is_subset(&second_half, &first_half) {
            return Some(row_index as i32);
        }
    }

    // Do something with rotated_matrix

    None
}

fn main() {
    let input: Vec<Vec<Vec<char>>> = include_str!("input_simple.txt")
        .split("\n\n")
        .map(|section| section.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let m: Vec<i32> = get_result(input);
    println!("{:?}", m);

    println!("{:?}", m.iter().sum::<i32>());
}

fn get_result(input: Vec<Vec<Vec<char>>>) -> Vec<i32> {
    let mut aa: Vec<i32> = Vec::new();
    for table in input.iter() {
        if let Some(row_index) = find_mirror(table) {
            aa.push(row_index*100);
        }

        let rotated_matrix = rotate_matrix(table);
        if let Some(col_index) = find_mirror(&rotated_matrix ) {
            aa.push(col_index);
        }
    }
    aa
}
