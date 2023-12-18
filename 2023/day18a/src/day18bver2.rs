use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i32,
    y: i32,
}

static DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn get_dig_coordinates(
    coordinates_list: &mut Vec<Coordinates>,
    direction: char,
    steps: i32,
    x: i32,
    y: i32,
) {
    let get_direction = DIRECTIONS[direction.to_digit(10).unwrap() as usize];
    coordinates_list.push(Coordinates {
        x: x + get_direction.0 * steps,
        y: y + get_direction.1 * steps,
    });
}

fn calculate_area(coordinates_list: &[Coordinates]) -> f64 {
    let coordinates_count = coordinates_list.len();
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..coordinates_count - 1 {
        sum1 += coordinates_list[i].x * coordinates_list[i + 1].y;
        sum2 += coordinates_list[i].y * coordinates_list[i + 1].x;
    }

    // First and last coordinates
    sum1 += coordinates_list[coordinates_count - 1].x * coordinates_list[0].y;
    sum2 += coordinates_list[0].x * coordinates_list[coordinates_count - 1].y;

    f64::abs((sum1 - sum2) as f64) / 2.0
}

fn calculate_inside(area: f64, coordinates_count: i32) -> i32 {
    (area - (coordinates_count / 2) as f64 + 1.0) as i32
}

fn main() {
    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);
        let mut coordinates_list = vec![Coordinates { x: 0, y: 0 }];
        let mut coordinates_count = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                let split_line: Vec<&str> = line.split_whitespace().collect();
                let last_part = split_line[2][2..].trim_end_matches(')');
                let direction = last_part.chars().last().unwrap();
                let steps = i32::from_str_radix(&last_part[..last_part.len() - 1], 16).unwrap();
                coordinates_count += steps;
                let last_coordinates = coordinates_list.last().unwrap();
                get_dig_coordinates(
                    &mut coordinates_list,
                    direction,
                    steps,
                    last_coordinates.x,
                    last_coordinates.y,
                );
            }
        }

        // Calculate inside area
        let area = calculate_area(&coordinates_list[..coordinates_list.len() - 1]);
        // Calculate points inside
        let area_points = calculate_inside(area, coordinates_count);
        // Add points from inside and outside
        let result = area_points + coordinates_count;
        println!("{}", result);
    }
}


// 40343619199142