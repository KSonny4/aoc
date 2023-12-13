fn count_arrangements(num: i32, row: &str, damaged_groups: &[usize]) -> usize {

    let mut index = 0;
    let group: &str = &(0..damaged_groups[index]).map(|_| "#").collect::<String>();
    let placement: &str = &(0..damaged_groups[index]).map(|_| "?").collect::<String>();


    if let Some(index) = row.find(placement) {
        // Replace the current unknown spring with a broken one
        let mut new_row = row.chars().collect::<Vec<_>>();
        new_row[index] = '#';
        let count1 = count_arrangements(num, &new_row.iter().collect::<String>(), damaged_groups);

        // Replace the current unknown spring with an operational one
        new_row[index] = '.';
        let count2 = count_arrangements(num, &new_row.iter().collect::<String>(), damaged_groups);

        count1 + count2
    } else {
        // All unknown springs are replaced, check if the arrangement is valid
        let row_str = row.chars().collect::<String>();
        is_valid_arrangement(num, &row_str, damaged_groups) as usize
    }
}


fn is_valid_arrangement(num: i32, row: &str, mut damaged_groups: &[usize]) -> bool {
    let mut consecutive_broken = 0;
    let mut group_index = 0;

    for c in row.chars() {
        match c {
            '#' => {
                consecutive_broken += 1;
            }
            '.' => {
                if consecutive_broken > 0 {
                    if let Some(group_size) = damaged_groups.get(group_index) {
                        if consecutive_broken != *group_size {
                            return false;
                        }
                        group_index += 1;
                    }
                    consecutive_broken = 0;
                }
            }
            _ => {
                // Ignore unknown springs
            }
        }
    }

    // Check if there are additional damaged groups at the end of the row
    while let Some(group_size) = damaged_groups.get(group_index) {
        if consecutive_broken != *group_size {
            return false;
        }
        group_index += 1;
    }

// Ensure that all damaged groups are covered
    if group_index == damaged_groups.len() && consecutive_broken == 0 {
        println!("{} Valid Row: {}",num, row);
        true
    } else {
        false
    }

}


fn main() {
    let input = "?###???????? 3,2,1
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    let total_arrangements: usize = input
        .lines()
        .enumerate()
        .map(|(index,line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0];
            let damaged_groups: Vec<usize> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            count_arrangements(index as i32, row, &damaged_groups)
        })
        .sum();

    println!("Total arrangements: {}", total_arrangements);
}

//
//
// fn count_arrangements(row: &str, damaged_groups: &[usize]) -> usize {
//     count_arrangements_recursive(row, damaged_groups, 0)
// }
//
// fn count_arrangements_recursive(row: &str, damaged_groups: &[usize], index: usize) -> usize {
//     if index == row.len() {
//         // Reached the end of the row, check if it's a valid arrangement
//         return if is_valid_arrangement(row, damaged_groups) { 1 } else { 0 };
//     }
//
//     let mut count = 0;
//
//     if row.chars().nth(index) == Some('?') {
//         // Try placing a broken spring
//         let mut new_row = row.chars().collect::<Vec<_>>();
//         new_row[index] = '#';
//         count += count_arrangements_recursive(&new_row.iter().collect::<String>(), damaged_groups, index + 1);
//
//         // Try placing an operational spring
//         new_row[index] = '.';
//         count += count_arrangements_recursive(&new_row.iter().collect::<String>(), damaged_groups, index + 1);
//     } else {
//         // Move to the next position in the row
//         count += count_arrangements_recursive(row, damaged_groups, index + 1);
//     }
//
//     count
// }
//
// fn is_valid_arrangement(row: &str, damaged_groups: &[usize]) -> bool {
//     let mut consecutive_broken = 0;
//     let mut group_index = 0;
//
//     for c in row.chars() {
//         match c {
//             '#' => {
//                 // If '#' is already present, continue without considering it
//                 if consecutive_broken > 0 {
//                     return false;
//                 }
//             }
//             '.' => {
//                 if consecutive_broken > 0 {
//                     if let Some(group_size) = damaged_groups.get(group_index) {
//                         if consecutive_broken != *group_size {
//                             return false;
//                         }
//                         group_index += 1;
//                     }
//                     consecutive_broken = 0;
//                 }
//             }
//             _ => {
//                 // Ignore unknown springs
//             }
//         }
//     }
//
//     // Check if there are additional damaged groups at the end of the row
//     while let Some(group_size) = damaged_groups.get(group_index) {
//         if consecutive_broken != *group_size {
//             return false;
//         }
//         group_index += 1;
//     }
//
//     // Ensure that all damaged groups are covered
//     group_index == damaged_groups.len() && consecutive_broken == 0
// }
//
// fn main() {
//     let input = "?###???????? 3,2,1
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
//
//     let total_arrangements: usize = input
//         .lines()
//         .map(|line| {
//             let parts: Vec<&str> = line.split_whitespace().collect();
//             let row = parts[0];
//             let damaged_groups: Vec<usize> = parts[1]
//                 .split(',')
//                 .filter_map(|s| s.parse().ok())
//                 .collect();
//             count_arrangements(row, &damaged_groups)
//         })
//         .sum();
//
//     println!("Total arrangements: {}", total_arrangements);
// }


// fn count_arrangements(num: i32, row: &str, damaged_groups: &[usize]) -> usize {
//     if let Some(index) = row.find('?') {
//         // Replace the current unknown spring with a broken one
//         let mut new_row = row.chars().collect::<Vec<_>>();
//         new_row[index] = '#';
//         let count1 = count_arrangements(num, &new_row.iter().collect::<String>(), damaged_groups);
//
//         // Replace the current unknown spring with an operational one
//         new_row[index] = '.';
//         let count2 = count_arrangements(num, &new_row.iter().collect::<String>(), damaged_groups);
//
//         count1 + count2
//     } else {
//         // All unknown springs are replaced, check if the arrangement is valid
//         let row_str = row.chars().collect::<String>();
//         is_valid_arrangement(num, &row_str, damaged_groups) as usize
//     }
// }
//
//
// fn is_valid_arrangement(num: i32, row: &str, mut damaged_groups: &[usize]) -> bool {
//     let mut consecutive_broken = 0;
//     let mut group_index = 0;
//
//     for c in row.chars() {
//         match c {
//             '#' => {
//                 consecutive_broken += 1;
//             }
//             '.' => {
//                 if consecutive_broken > 0 {
//                     if let Some(group_size) = damaged_groups.get(group_index) {
//                         if consecutive_broken != *group_size {
//                             return false;
//                         }
//                         group_index += 1;
//                     }
//                     consecutive_broken = 0;
//                 }
//             }
//             _ => {
//                 // Ignore unknown springs
//             }
//         }
//     }
//
//     // Check if there are additional damaged groups at the end of the row
//     while let Some(group_size) = damaged_groups.get(group_index) {
//         if consecutive_broken != *group_size {
//             return false;
//         }
//         group_index += 1;
//     }
//
// // Ensure that all damaged groups are covered
//     if group_index == damaged_groups.len() && consecutive_broken == 0 {
//         println!("{} Valid Row: {}",num, row);
//         true
//     } else {
//         false
//     }
//
// }
//
//
// fn main() {
//     let input = "?###???????? 3,2,1
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
//
//     let total_arrangements: usize = input
//         .lines()
//         .enumerate()
//         .map(|(index,line)| {
//             let parts: Vec<&str> = line.split_whitespace().collect();
//             let row = parts[0];
//             let damaged_groups: Vec<usize> = parts[1]
//                 .split(',')
//                 .filter_map(|s| s.parse().ok())
//                 .collect();
//             count_arrangements(index as i32, row, &damaged_groups)
//         })
//         .sum();
//
//     println!("Total arrangements: {}", total_arrangements);
// }
//
// //
// //
// // fn count_arrangements(row: &str, damaged_groups: &[usize]) -> usize {
// //     count_arrangements_recursive(row, damaged_groups, 0)
// // }
// //
// // fn count_arrangements_recursive(row: &str, damaged_groups: &[usize], index: usize) -> usize {
// //     if index == row.len() {
// //         // Reached the end of the row, check if it's a valid arrangement
// //         return if is_valid_arrangement(row, damaged_groups) { 1 } else { 0 };
// //     }
// //
// //     let mut count = 0;
// //
// //     if row.chars().nth(index) == Some('?') {
// //         // Try placing a broken spring
// //         let mut new_row = row.chars().collect::<Vec<_>>();
// //         new_row[index] = '#';
// //         count += count_arrangements_recursive(&new_row.iter().collect::<String>(), damaged_groups, index + 1);
// //
// //         // Try placing an operational spring
// //         new_row[index] = '.';
// //         count += count_arrangements_recursive(&new_row.iter().collect::<String>(), damaged_groups, index + 1);
// //     } else {
// //         // Move to the next position in the row
// //         count += count_arrangements_recursive(row, damaged_groups, index + 1);
// //     }
// //
// //     count
// // }
// //
// // fn is_valid_arrangement(row: &str, damaged_groups: &[usize]) -> bool {
// //     let mut consecutive_broken = 0;
// //     let mut group_index = 0;
// //
// //     for c in row.chars() {
// //         match c {
// //             '#' => {
// //                 // If '#' is already present, continue without considering it
// //                 if consecutive_broken > 0 {
// //                     return false;
// //                 }
// //             }
// //             '.' => {
// //                 if consecutive_broken > 0 {
// //                     if let Some(group_size) = damaged_groups.get(group_index) {
// //                         if consecutive_broken != *group_size {
// //                             return false;
// //                         }
// //                         group_index += 1;
// //                     }
// //                     consecutive_broken = 0;
// //                 }
// //             }
// //             _ => {
// //                 // Ignore unknown springs
// //             }
// //         }
// //     }
// //
// //     // Check if there are additional damaged groups at the end of the row
// //     while let Some(group_size) = damaged_groups.get(group_index) {
// //         if consecutive_broken != *group_size {
// //             return false;
// //         }
// //         group_index += 1;
// //     }
// //
// //     // Ensure that all damaged groups are covered
// //     group_index == damaged_groups.len() && consecutive_broken == 0
// // }
// //
// // fn main() {
// //     let input = "?###???????? 3,2,1
// // .??..??...?##. 1,1,3
// // ?#?#?#?#?#?#?#? 1,3,1,6
// // ????.#...#... 4,1,1
// // ????.######..#####. 1,6,5
// // ?###???????? 3,2,1";
// //
// //     let total_arrangements: usize = input
// //         .lines()
// //         .map(|line| {
// //             let parts: Vec<&str> = line.split_whitespace().collect();
// //             let row = parts[0];
// //             let damaged_groups: Vec<usize> = parts[1]
// //                 .split(',')
// //                 .filter_map(|s| s.parse().ok())
// //                 .collect();
// //             count_arrangements(row, &damaged_groups)
// //         })
// //         .sum();
// //
// //     println!("Total arrangements: {}", total_arrangements);
// // }
