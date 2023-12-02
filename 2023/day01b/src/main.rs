use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Mapping {
    number: u32,
    location: u32,
}

fn get_mapping() -> HashMap<String, i32> {
    let mut word_to_num_map = HashMap::new();

    // Create keys from one to ten as words
    word_to_num_map.insert("one".to_string(), 1);
    word_to_num_map.insert("two".to_string(), 2);
    word_to_num_map.insert("three".to_string(), 3);
    word_to_num_map.insert("four".to_string(), 4);
    word_to_num_map.insert("five".to_string(), 5);
    word_to_num_map.insert("six".to_string(), 6);
    word_to_num_map.insert("seven".to_string(), 7);
    word_to_num_map.insert("eight".to_string(), 8);
    word_to_num_map.insert("nine".to_string(), 9);
    word_to_num_map.insert("ten".to_string(), 10);

    // Create keys from one to ten as numerics
    for i in 1..=10 {
        word_to_num_map.insert(i.to_string(), i);
    }
    word_to_num_map
}

fn string_to_sum(input: &str) -> i32 {
    let mut found_numbers: Vec<_> = Vec::new();
    let mapping = get_mapping();

    for (word, value) in mapping.iter() {
        let found_numbers_tuple: Vec<_> = input
            .match_indices(word)
            .map(|(index, _)| Mapping { location: index as u32, number: *value as u32 })
            .collect();
        if !found_numbers_tuple.is_empty() {
            found_numbers.extend(found_numbers_tuple.clone());
        }
        // println!("{:?}", a);
    }

    found_numbers.sort_by_key(|item| item.location);
    println!("{:?}", found_numbers);

    let first_element = found_numbers
        .first()
        .map(|item| format!("{:?}", item.number))
        .unwrap_or_default();
    let last_element = found_numbers
        .last()
        .map(|item| format!("{:?}", item.number))
        .unwrap_or_default();

    format!("{}{}", first_element, last_element)
        .parse::<i32>()
        .unwrap_or(0)
}

fn main() {
    let res: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| string_to_sum(line))
        .collect();

    let sum: i32 = res.iter().sum();
    println!("Sum: {}", sum);
    // for n in &res {
    //     println!("{:?}", n);
    // }
}
