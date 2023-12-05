use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    source_category: i64,
    destination_category: i64,
    source_range: i64,
}


fn main() {
    let data: Vec<Vec<String>> = include_str!("input.txt")
        .split("\n\n")
        .map(|section| section.lines().map(String::from).collect())
        .collect();

    let seeds: Vec<i64> = data[0][0]
        .split(':')
        .nth(1)
        .map(|nums| nums.split_whitespace().filter_map(|num| num.parse().ok()).collect())
        .unwrap_or_default();

    let num_maps: Vec<Vec<Mapping>> = data[1..].iter().map(|_map| {
        _map[1..].iter().map(|mapping| {
            let nums: Vec<i64> = mapping.split_whitespace().filter_map(|num| num.parse().ok()).collect();
            match nums.as_slice() {
                [destination, source, range] => Mapping {
                    destination_category: *destination,
                    source_category: *source,
                    source_range: source + range,
                },
                _ => panic!("Invalid values"),
            }
        }).collect()
    }).collect();

    println!("{:#?}", num_maps);
    println!("{:?}", seeds);

    let mut seed_locs: Vec<i64> = Vec::new();
    seeds.iter().for_each(|seed| {
        let end_of_mapping = num_maps.len();


        let mut index = 0;
        let mut current_seed = *seed;

        while index < end_of_mapping {
            let mut changed = false;
            for mapping in &num_maps[index] {
                if mapping.source_category.contains(&current_seed) {
                    let seed_distance_from_start = current_seed - mapping.source_category.start; // N numbers from start
                    current_seed = seed_distance_from_start + mapping.destination_category.start; // desti
                    index += 1;
                    changed = true;
                    break;
                }
            }
            if !changed{
                index += 1;
            }
        }

        seed_locs.push(current_seed);
        println!("{}", current_seed);

    });

    let seed_locs: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            let mut current_seed = *seed;
            for mappings in &num_maps {
                for mapping in mappings {
                    if &current_seed >= &mapping.source_category && &current_seed < &mapping.source_range {
                        let seed_distance_from_start = current_seed - mapping.source_category;
                        current_seed = seed_distance_from_start + mapping.destination_category;
                        break;
                    }
                }
            }
            current_seed
        })
        .collect();


    if let Some(min_value) = seed_locs.iter().cloned().min() {
        println!("The minimum value is: {}", min_value);
    } else {
        println!("The vector is empty.");
    }
}
