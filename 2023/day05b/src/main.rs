use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug)]
struct Mapping {
    source_category: i64,
    destination_category: i64,
    source_range: i64,
}

fn main() {
    let start_time = Instant::now();

    let data: Vec<Vec<String>> = include_str!("input.txt")
        .split("\n\n")
        .map(|section| section.lines().map(String::from).collect())
        .collect();

    let seeds: Vec<i64> = data[0][0]
        .split(':')
        .nth(1)
        .map(|nums| nums.split_whitespace().filter_map(|num| num.parse().ok()).collect())
        .unwrap_or_default();

    let mut seeds_from_ranges: Vec<i64> = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        let x = seeds[i];
        let y = seeds[i + 1];
        for gg in x..x + y {
            seeds_from_ranges.push(gg);
        }
    }




    println!("Seeds: {:?}", seeds_from_ranges.len());

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time Seeds: {:?}", elapsed_time);

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
    let seed_locs: Vec<i64> = seeds_from_ranges
        .par_iter()
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


    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
}
