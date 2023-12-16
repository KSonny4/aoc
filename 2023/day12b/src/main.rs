
use std::collections::HashMap;

fn check_result(row: &Vec<char>, damaged_groups: &Vec<i64>, memoization: &mut HashMap<(Vec<char>, Vec<i64>), i64>) -> i64 {
    memoization
        .get(&(row.clone(), damaged_groups.clone()))
        .cloned()
        .unwrap_or_else(|| {
            match (damaged_groups.is_empty(), damaged_groups.get(0).map(|&x| x as usize)) {
                (true, _) => 0,
                (_, Some(groups)) if row.len() < groups => 0,
                (_, Some(groups)) if row.iter().take(groups).any(|&c| c == '.') => 0,
                (_, Some(groups)) if row.len() == groups => {
                    if damaged_groups.len() == 1 {
                        1
                    } else {
                        0
                    }
                }
                (_, Some(groups)) if row[groups] == '#' => 0,
                (_, Some(groups)) => {
                    let result = count_arrangements2(&row[(groups + 1)..].to_vec(), &damaged_groups[1..].to_vec(), memoization);
                    memoization.insert((row.clone(), damaged_groups.clone()), result);
                    result
                }
                _ => 0,
            }
        })
}

fn count_arrangements2(row: &Vec<char>, damaged_groups: &Vec<i64>, memoization: &mut HashMap<(Vec<char>, Vec<i64>), i64>) -> i64 {
    // we are at the end
    if row.is_empty() {
        if damaged_groups.is_empty() {
            return 1
        }
        else {
            return 0
        }
    }
    match row[0] {
        '.' => {
            return count_arrangements2(&row[1..].to_vec(), damaged_groups, memoization);
        }
        '#' => {
            return check_result(row, damaged_groups, memoization);
        }
        '?' => {
            return count_arrangements2(&row[1..].to_vec(), damaged_groups, memoization)
                + check_result(row, damaged_groups, memoization);
        }
        _ => panic!("Unexpected character!!!!!!")
    }
}


fn main() {

    // ???.### 1,1,3
    // ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
    let mut memoization = HashMap::new();
    let total_arrangements: Vec<i64> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0].to_string();

            let repeated_row = std::iter::repeat(row).take(5).collect::<Vec<String>>().join("?");

            let damaged_groups: Vec<i64> = parts[1].split(',').filter_map(|s| s.parse().ok()).collect();
            let repeated_groups: Vec<Vec<i64>> = std::iter::repeat(damaged_groups).take(5).collect();
            let flattened_groups: Vec<i64> = repeated_groups.into_iter().flatten().collect();


            let res = count_arrangements2(  &repeated_row.chars().collect(), &flattened_groups, &mut memoization);
            // println!("index: {}, res: {}", index, res);
            res
        })
        .collect();
    println!("Total arrangements: {:?}", total_arrangements);
    println!("Total arrangements: {}", total_arrangements.iter().sum::<i64>());
}
