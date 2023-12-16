use std::env::join_paths;
use regex::Regex;

fn replace_char_at_index(input: &str, index: usize, replacement: char) -> String {
    let mut modified = input.chars().collect::<Vec<_>>();
    if let Some(element) = modified.get_mut(index) {
        *element = replacement;
    }

    modified.into_iter().collect()
}
fn count_groups(input: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current_group = 0;

    for c in input.chars() {
        match c {
            '#' => current_group += 1,
            '.' => {
                if current_group > 0 {
                    result.push(current_group);
                    current_group = 0;
                }
            }
            _ => {} // Ignore other characters
        }
    }

    // If the input ends with "#", add the last group
    if current_group > 0 {
        result.push(current_group);
    }

    result
}

fn check_result(row: &str, damaged_groups_original: &[i32]) -> i32 {
    let res = count_groups(row);
    //println!("row: {:?}, res: {:?}, damaged_groups_original: {:?}", row, res, damaged_groups_original);
    if res == damaged_groups_original.to_vec() {
        1
    }
    else {
        0
    }
}
fn count_arrangements2(index: usize, row: &str, original: &str, damaged_groups_original: &[i32]) -> i32 {
    // we are at the end
    if index > row.len() - 1 {
        return check_result(row, damaged_groups_original);
    }
    let element = row.chars().nth(index).unwrap();
    match element {
        '?' => {
            let modify_row_dot = replace_char_at_index(row, index, '.');
            let modify_row_hash = replace_char_at_index(row, index, '#');
            return count_arrangements2(index + 1, &modify_row_dot, original, damaged_groups_original)
                + count_arrangements2(index + 1, &modify_row_hash,  original, damaged_groups_original);
        }
        _ => {
            return count_arrangements2(index + 1, row, original, damaged_groups_original);
        }
    }
}


fn main() {

    // ???.### 1,1,3
    // ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
    let total_arrangements: Vec<i32> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0].to_string();
            //
            // let mut r: Vec<String> = Vec::new();
            // for i in 0..5 {
            //     r.push(row.clone());
            // }
            //
            // let aa = r.join("?");
            let repeated_row = std::iter::repeat(row).take(5).collect::<Vec<String>>().join("?");

            let damaged_groups: Vec<i32> = parts[1].split(',').filter_map(|s| s.parse().ok()).collect();
            let repeated_groups: Vec<Vec<i32>> = std::iter::repeat(damaged_groups).take(5).collect();
            let flattened_groups: Vec<i32> = repeated_groups.into_iter().flatten().collect();




            count_arrangements2(0, &repeated_row, &repeated_row, &flattened_groups)
        })
        .collect();
    println!("Total arrangements: {:?}", total_arrangements);
    println!("Total arrangements: {}", total_arrangements.iter().sum::<i32>());
}
