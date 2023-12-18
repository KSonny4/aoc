use regex::Regex;

#[derive(Debug)]
struct Action {
    direction: Direction,
    value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i64,
    y: i64,
}

fn char_to_direction(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("Invalid direction character: {}", c),
    }
}


fn get_dig_coordinates(
    coordinates_list: &mut Vec<Coordinates>,
    direction: Direction,
    steps: i64,
    x: i64,
    y: i64,
) {


    let direction = match direction {
        Direction::Left => ( -1, 0),
        Direction::Right => ( 1, 0),
        Direction::Up => (0 , -1),
        Direction::Down => (0 ,  1),
    };


    coordinates_list.push(Coordinates {
        x: x + direction.0 * steps,
        y: y + direction.1 * steps,
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

fn calculate_inside(area: f64, coordinates_count: i64) -> i64 {
    (area - (coordinates_count / 2) as f64 + 1.0) as i64
}


fn main() {
    let mut coordinates_list = vec![Coordinates { x: 0, y: 0 }];
    let mut coordinates_count = 0;

    let input = include_str!("input.txt");
    let re = Regex::new(r"([RDLU]) (\d+) \((#[0-9a-fA-F]{6})\)").unwrap();
    re
        .captures_iter(input)
        .for_each(|cap|  {
            let steps = cap[2].parse().unwrap();
            coordinates_count += steps;
            let binding = coordinates_list.clone();
            let last_coordinates = binding.last().unwrap();get_dig_coordinates(
                &mut coordinates_list,
                char_to_direction(cap[1].chars().next().unwrap()),
                steps,
                last_coordinates.x,
                last_coordinates.y,
            );

        });

    // Calculate inside area
    let area = calculate_area(&coordinates_list.clone());
    // Calculate points inside
    let area_points = calculate_inside(area, coordinates_count);
    // Add points from inside and outside
    let result = area_points + coordinates_count;
    println!("{}", result);
}
