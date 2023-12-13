//
//
// #[derive(Debug, Clone)]
// struct Input {
//     record: String,
//     arrangement: Vec<i32>
// }
// fn reset_array(array: &mut Vec<char>, start_index: usize, end_index: usize) {
//     for i in start_index..=end_index {
//         array[i] = if array[i] == '?' || array[i] == '.' { '?' } else { '.' };
//     }
// }
//
// fn count_possibilities(array: &mut Vec<char>, substrings: &[Vec<char>], index: usize) -> usize {
//     if index == array.len() {
//         // Base case: All positions are filled, check if the array is a valid placement
//         let string_representation: String = array.iter().collect();
//         if substrings.iter().all(|sub| string_representation.contains(&sub.iter().collect::<String>())) {
//             println!("{}", string_representation);
//             return 1;
//         }
//         return 0;
//     }
//
//     let mut possibilities = 0;
//
//     // Try placing substrings at the current index with dots between blocks
//     for sub in substrings {
//         if index + sub.len() + sub.len() - 1 <= array.len() {
//             let valid_placement = sub.iter().enumerate().all(|(i, &c)| array[index + i * 2] == '?' || array[index + i * 2] == c);
//
//             if valid_placement {
//                 // Place the substring with dots between blocks
//                 for (i, &c) in sub.iter().enumerate() {
//                     array[index + i * 2] = c;
//                     if i < sub.len() - 1 {
//                         array[index + i * 2 + 1] = '.';
//                     }
//                 }
//
//                 // Move to the next position after the last dot
//                 possibilities += count_possibilities(array, substrings, index + sub.len() * 2);
//
//                 // Reset the array for backtracking
//                 reset_array(array, index, index + sub.len() * 2 - 1);
//             }
//         }
//     }
//
//     possibilities
// }
//
// fn main() {
//     let array: Vec<char> = "?###????????".chars().collect();
//     let substrings: Vec<Vec<char>> = vec![vec!['#'], vec!['#'], vec!['#', '#', '#']];
//
//     let mut mutable_array = array.clone();
//     let possibilities = count_possibilities(&mut mutable_array, &substrings, 0);
//     println!("Total possibilities: {}", possibilities);
// }
//
//
// //
// // fn main() {
// //     let input: Vec<Input> = include_str!("input_simple.txt")
// //         .lines()
// //         .map(|line| {
// //             let mut split_ = line.split_whitespace(); // Use split_whitespace for more robust splitting
// //
// //             let first_part = split_.next().unwrap().to_string();
// //             let second_part: Vec<i32> = split_
// //                 .next()
// //                 .unwrap()
// //                 .split(',')
// //                 .map(|x| x.parse::<i32>().unwrap())
// //                 .collect();
// //
// //             Input {record: first_part, arrangement: second_part}
// //         })
// //         .collect();
// //
// //     println!("{:?}", input);
// // }


fn reset_array(array: &mut Vec<char>, start_index: usize, end_index: usize) {
    for i in start_index..=end_index {
        array[i] = if array[i] == '?' || array[i] == '.' { '?' } else { '.' };
    }
}

fn count_possibilities(array: &mut Vec<char>, substrings: &[Vec<char>], index: usize) -> usize {
    if index >= array.len() {
        // Base case: All positions are filled, check if the array is a valid placement
        let string_representation: String = array.iter().collect();
        if substrings.iter().all(|sub| string_representation.contains(&sub.iter().collect::<String>())) {
            println!("{}", string_representation);
            return 1;
        }
        return 0;
    }

    let mut possibilities = 0;

    // Try placing substrings at the current index with dots between blocks
    for sub in substrings {
        if index + sub.len() + sub.len() - 1 <= array.len() {
            let valid_placement = sub.iter().enumerate().all(|(i, &c)| array[index + i * 2] == '?' || array[index + i * 2] == c);

            if valid_placement {
                // Place the substring with dots between blocks
                for (i, &c) in sub.iter().enumerate() {
                    array[index + i * 2] = c;
                    if i < sub.len() - 1 {
                        array[index + i * 2 + 1] = '.';
                    }
                }

                // Move to the next position after the last dot
                possibilities += count_possibilities(array, substrings, index + sub.len() * 2);

                // Reset the array for backtracking
                reset_array(array, index, index + sub.len() * 2 - 1);
            }
        }
    }

    possibilities
}

fn main() {
    let array: Vec<char> = "?###????????".chars().collect();
    let substrings: Vec<Vec<char>> = vec![vec!['#'], vec!['#'], vec!['#', '#', '#']];

    let mut mutable_array = array.clone();
    let possibilities = count_possibilities(&mut mutable_array, &substrings, 0);
    println!("Total possibilities: {}", possibilities);
}
