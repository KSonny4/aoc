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

fn is_one_change_away(vec1: String, vec2: String) -> bool {
    // Check if the vectors have the same length
    if vec1.len() != vec2.len() {
        return false;
    }

    // Count the number of differing positions
    let mut diff_count = 0;
    for (c1, c2) in vec1.chars().zip(vec2.chars()) {
        if c1 != c2 {
            diff_count += 1;
        }

        // If more than one difference is found, return false
        if diff_count > 1 {
            return false;
        }
    }

    // Return true if only one difference is found
    diff_count == 1
}


fn is_subset(iter_: &Vec<String>, other: &Vec<String>) -> bool {
    let mut one_change_allowed_counter = 0;
    for (one, two) in other.iter().rev().cloned().zip(iter_) {
        if one != two.clone(){
            if is_one_change_away(one, two.clone()) {
                one_change_allowed_counter += 1;
            } else {
                return false
            }

            if one_change_allowed_counter > 1 {
                return false
            }
        }


    }
    one_change_allowed_counter == 1
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

        if is_subset(&second_half, &first_half) {
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
            find_mirror(&rotate_matrix(&table))
        }))
        .collect();

    println!("{:?}", result);
    println!("{:?}", result.iter().sum::<i32>());
}
