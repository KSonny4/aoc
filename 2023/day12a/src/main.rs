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
    println!("row: {:?}, res: {:?}, damaged_groups_original: {:?}", row, res, damaged_groups_original);
    if res == damaged_groups_original.to_vec() {
        1
    }
    else {
        0
    }
}

fn count_arrangements(index: usize, row: &str, damaged_groups: &[i32], original: &str, damaged_groups_original: &[i32]) -> i32 {
    // check if placement correct
    if damaged_groups.is_empty() {
        return check_result(row, damaged_groups_original);
    }

    // we are at the end
    if index > row.len() - 1 {
        return check_result(row, damaged_groups_original);
    }

    let element = row.chars().nth(index).unwrap();

    match element {
        '#' => {
            let damage_group = damaged_groups[0] - 1;

            match damage_group {
                0 => {
                    // The group is finished, remove it from damaged_groups and move index
                    return count_arrangements(index + 1, row, &damaged_groups[1..], original, damaged_groups_original);
                }
                -1 => return 0, // we touched wrong group
                _ => {
                    // Decrement the group and continue
                    let mut aa = Vec::new();
                    aa.push(damage_group);
                    // Add damaged_groups[1:] to aa
                    aa.extend_from_slice(&damaged_groups[1..]);
                    return count_arrangements(index + 1, row, &aa, original, damaged_groups_original);
                }
            }
        }
        '?' => {
            let modify_row_dot = replace_char_at_index(row, index, '.');
            let modify_row_hash = replace_char_at_index(row, index, '#');
            // Place "#" or "."

            let damage_group = damaged_groups[0] - 1;
            // Decrement the group and continue
            let mut aa = Vec::new();
            aa.push(damage_group);
            // Add damaged_groups[1:] to aa
            aa.extend_from_slice(&damaged_groups[1..]);

            // move index, no change in group
            let count_with_dot =
                count_arrangements(index + 1, &modify_row_dot, &damaged_groups, original, damaged_groups_original);
            // move index, decrement element in group
            let count_with_hash =
                count_arrangements(index + 1, &modify_row_hash, &aa, original, damaged_groups_original);
            return count_with_dot + count_with_hash;
        }
        '.' => {
            return count_arrangements(index + 1, row, damaged_groups, original, damaged_groups_original);
        }
        _ => 0, // Handle other cases if necessary
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
    let input = "?###???????? 3,2,1";
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    //let input = "????.######..#####. 1,6,5";
    let total_arrangements: Vec<i32> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0].to_string();
            let damaged_groups: Vec<i32> = parts[1].split(',').filter_map(|s| s.parse().ok()).collect();
            count_arrangements2(0, &row, &row, &damaged_groups)
        })
        .collect();

    println!("Total arrangements: {:?}", total_arrangements);
    println!("Total arrangements: {}", total_arrangements.iter().sum::<i32>());
}
