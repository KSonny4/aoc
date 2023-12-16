//
// fn count_arrangements(num: i32, row: &str, damaged_groups: &Vec<usize>, original: &str, damaged_groups_original: &Vec<usize>) -> i32 {
//     let index = 0;
//     let dmg_group_len = damaged_groups[index];
//     let group: Vec<char> = (0..dmg_group_len).map(|_| '#').collect();
//     let placement: Vec<char> = (0..dmg_group_len).map(|_| '?').collect();
//
//
//     if let Some(index) = row.find(placement.iter().collect::<String>().as_str()) {
//         let mut placement_was_possible = true;
//         // check that index -1 and index + 1 are either "." or "?"
//         let mut new_row: Vec<char> = row.chars().collect();
//         if index as i32 - 1 >= 0 {
//             if new_row[index - 1] == '#' {
//                 // placement ruined, we need to place "." there
//                 new_row[index] = '.';
//                 placement_was_possible = false;
//             }
//         }
//
//         if index + 1 < row.len() {
//             if new_row[index + 1] == '#' {
//                 new_row[index] = '.';
//                 placement_was_possible = false;
//             }
//
//         }
//
//         // replace the damaged group with the new group
//         let mut count1 = 0;
//         if placement_was_possible {
//             new_row.splice(index..index + dmg_group_len, group.iter().cloned());
//             count1 = count_arrangements(num, &new_row.iter().collect::<String>(), &damaged_groups[1..].to_vec(), original, damaged_groups_original);
//         }
//
//         //let replaced_str = replace_first(row.clone(), "?", ".");
//         let count2 = count_arrangements(num, &new_row.iter().collect::<String>(), &damaged_groups, original, damaged_groups_original);
//
//         let res = count1 + count2;
//         println!("{}: {} -> {} -> {} -> {}", num, row, new_row.iter().collect::<String>(), damaged_groups.len(), res);
//         res
//     } else {
//
//         // todo check co se vybralo z damaged_groups a co se nevybralo by melo bejt uz zadratovane tady
//         let mut r = row.to_string();
//
//         //create regex from damaged_groups [3,2,1] -> "\.*###\.*##\.*#\.*
//         let mut regex_str = String::new();
//         for dmg_group_len in damaged_groups_original {
//             regex_str.push_str(&format!("\\.*{}", "#".repeat(*dmg_group_len)));
//         }
//         regex_str.push_str("\\.*");
//
//         let re = regex::Regex::new(&regex_str).unwrap();
//         let res = re.find_iter(&*r).count() as i32;
//         res
//     }
// }
//
// fn main() {
// //     let input = "???.### 1,1,3
// // .??..??...?##. 1,1,3
// // ?#?#?#?#?#?#?#? 1,3,1,6
// // ????.#...#... 4,1,1
// // ????.######..#####. 1,6,5
// // ?###???????? 3,2,1";
//     // let input = "????.#...#... 4,1,1";
//      //let input = "?###???????? 3,2,1";
//
//     let input = "???.### 1,1,3";
//     let total_arrangements: Vec<i32> = input
//         .lines()
//         .enumerate()
//         .map(|(index, line)| {
//             let parts: Vec<&str> = line.split_whitespace().collect();
//             let row = parts[0];
//             let damaged_groups: Vec<usize> = parts[1]
//                 .split(',')
//                 .filter_map(|s| s.parse().ok())
//                 .collect();
//             count_arrangements(index as i32, row, &damaged_groups, row, &damaged_groups,)
//         })
//         .collect();
//
//     println!("Total arrangements: {:?}", total_arrangements);
//
//     println!("Total arrangements: {}", total_arrangements.iter().sum::<i32>());
// }



fn count_arrangements(index: i32, row: &str, damaged_groups: &Vec<usize>, original: &str, damaged_groups_original: &Vec<usize>) -> i32 {
    let dmg_group_len = damaged_groups[index];

    let element = row.as_bytes()[index as usize];



}

fn main() {
//     let input = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    // let input = "????.#...#... 4,1,1";
    //let input = "?###???????? 3,2,1";

    let input = "???.### 1,1,3";
    let total_arrangements: Vec<i32> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0];
            let damaged_groups: Vec<usize> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            count_arrangements(0, row, &damaged_groups, row, &damaged_groups,)
        })
        .collect();

    println!("Total arrangements: {:?}", total_arrangements);

    println!("Total arrangements: {}", total_arrangements.iter().sum::<i32>());
}
