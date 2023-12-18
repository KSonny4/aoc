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

fn calculate(actions: Vec<Action>) {
    let mut location: (i64, i64) = (0, 0);
    let mut sum1: i64 = 0;
    let mut sum2: i64 = 0;
    let mut sum_dir: i64 = 0;

    for action in &actions {
        let mut new_location = location.clone();

        let direction = match action.direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };

        new_location = (
            location.0 + direction.0 * action.value,
            location.1 + direction.1 * action.value,
        );
        sum1 = sum1 + (location.0) * (new_location.1);
        sum2 = sum2 + (location.1) * (new_location.0);
        sum_dir += action.value;
        location = new_location;
    }

    let area = (sum1 - sum2).abs() / 2;
    println!("area: {:?}", area + sum_dir / 2 + 1);
}

fn main() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"([RDLU]) (\d+) \((#[0-9a-fA-F]{6})\)").unwrap();
    let hexa_to_int = |s: &str| i64::from_str_radix(s, 16).unwrap();
    let actions: Vec<Action> = re
        .captures_iter(input)
        .map(|cap| {
            let hex_color = &cap[3][1..6];
            let value = hexa_to_int(hex_color);

            let direction_char = &cap[3][6..7];
            let direction = char_to_direction(direction_char.chars().next().unwrap());

            Action {
                direction,
                value,
            }
        })
        .collect();

    for action in &actions {
        println!("{:?}", action);
    }

    calculate(actions);
}
