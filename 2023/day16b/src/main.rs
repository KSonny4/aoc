use std::collections::{HashMap, HashSet};
use std::fmt::format;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BeamPoint {
    position: (usize, usize),
    came_from_direction: Direction,
    symbol: char,
}



// Implement Debug trait for BeamPoint
impl std::fmt::Debug for BeamPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BeamPoint {{ position: {:?}, direction: {:?}, symbol: '{}' }}",
            self.position, self.came_from_direction, self.symbol
        )
    }
}

fn move_in_direction(new_direction: Direction, beampoint: &BeamPoint, data: &Vec<Vec<char>>, dx: isize, dy: isize) -> Option<BeamPoint> {
    let (x, y) = beampoint.position;
    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;

    if new_x < data.len() && new_y < data[0].len() {
        let new_symbol = data[new_x][new_y];
        Some(BeamPoint {
            position: (new_x, new_y),
            came_from_direction: new_direction,
            symbol: new_symbol,
        })
    } else {
        None
    }
}

fn move_and_push(new_direction: Direction, beampoint: &BeamPoint, data: &Vec<Vec<char>>, dx: isize, dy: isize, queue: &mut Vec<BeamPoint>, history: &HashSet<BeamPoint>) {
    if let Some(new_beampoint) = move_in_direction(new_direction, beampoint, data, dx, dy) {
        if !history.contains(&new_beampoint) {
            queue.push(new_beampoint);
        }
    }
}

fn beam_travel(data: &Vec<Vec<char>>, start: BeamPoint ) -> HashSet<BeamPoint> {
    let mut history: HashSet<BeamPoint> = HashSet::new();
    let mut queue: Vec<BeamPoint> = Vec::new();
    queue.push(start);

    while let Some(beampoint) = queue.pop() {
        let beampoint_string: String = format!("{:?}", beampoint);
        history.insert(beampoint);

        match beampoint.symbol {
            '.' => match beampoint.came_from_direction {
                Direction::Down => move_and_push(Direction::Down, &beampoint, data, 1, 0, &mut queue, &history),
                Direction::Right => move_and_push(Direction::Right, &beampoint, data, 0, 1, &mut queue, &history),
                Direction::Left => move_and_push(Direction::Left, &beampoint, data, 0, -1, &mut queue, &history),
                Direction::Up => move_and_push(Direction::Up, &beampoint, data, -1, 0, &mut queue, &history),
                _ => panic!("Unexpected beampoint: {:?}", beampoint),
            },
            '|' => match beampoint.came_from_direction {
                Direction::Down => move_and_push(Direction::Down, &beampoint, data, 1, 0, &mut queue, &history),
                Direction::Up => move_and_push(Direction::Up, &beampoint, data, -1, 0, &mut queue, &history),
                Direction::Right | Direction::Left => {
                    move_and_push(Direction::Down, &beampoint, data, 1, 0, &mut queue, &history);
                    move_and_push(Direction::Up, &beampoint, data, -1, 0, &mut queue, &history);
                }
                _ => panic!("Unexpected beampoint: {:?}", beampoint),
            },
            '-' => match beampoint.came_from_direction {
                Direction::Right => move_and_push(Direction::Right, &beampoint, data, 0, 1, &mut queue, &history),
                Direction::Left => move_and_push(Direction::Left, &beampoint, data, 0, -1, &mut queue, &history),
                Direction::Up | Direction::Down => {
                    move_and_push(Direction::Right, &beampoint, data, 0, 1, &mut queue, &history);
                    move_and_push(Direction::Left, &beampoint, data, 0, -1, &mut queue, &history);
                }
            },
            '\\' => match beampoint.came_from_direction {
                Direction::Down => move_and_push(Direction::Right, &beampoint, data, 0, 1, &mut queue, &history),
                Direction::Right => move_and_push(Direction::Down, &beampoint, data, 1, 0, &mut queue, &history),
                Direction::Left => move_and_push(Direction::Up, &beampoint, data, -1, 0, &mut queue, &history),
                Direction::Up => move_and_push(Direction::Left, &beampoint, data, 0, -1, &mut queue, &history),
                _ => panic!("Unexpected beampoint: {:?}", beampoint),
            },
            '/' => match beampoint.came_from_direction {
                Direction::Down => move_and_push(Direction::Left, &beampoint, data, 0, -1, &mut queue, &history),
                Direction::Right => move_and_push(Direction::Up, &beampoint, data, -1, 0, &mut queue, &history),
                Direction::Left => move_and_push(Direction::Down, &beampoint, data, 1, 0, &mut queue, &history),
                Direction::Up => move_and_push(Direction::Right, &beampoint, data, 0, 1, &mut queue, &history),
                _ => panic!("Unexpected beampoint: {:?}", beampoint),
            },
            _ => panic!("Unexpected beampoint: {:?}", beampoint),
        }

    }

    history
}

fn main() {
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();


    let rows = data.len();
    let cols = data[0].len();

    let top_row_starts: Vec<BeamPoint> = (0..cols).map(|i| BeamPoint { position: (0, i), came_from_direction: Direction::Down, symbol: data[0][i] }).collect();
    let bottom_row_starts: Vec<BeamPoint> = (0..cols).map(|i| BeamPoint { position: (rows - 1, i), came_from_direction: Direction::Up, symbol: data[rows - 1][i] }).collect();
    let left_column_starts: Vec<BeamPoint> = (0..rows).map(|i| BeamPoint { position: (i, 0), came_from_direction: Direction::Right, symbol: data[i][0] }).collect();
    let right_column_starts: Vec<BeamPoint> = (0..rows).map(|i| BeamPoint { position: (i, cols - 1), came_from_direction: Direction::Left, symbol: data[i][cols - 1] }).collect();

    let all_possible_starts: Vec<BeamPoint> = top_row_starts
        .into_iter()
        .chain(bottom_row_starts.into_iter())
        .chain(left_column_starts.into_iter())
        .chain(right_column_starts.into_iter())
        .collect();


    let res = all_possible_starts.iter().map(
        |start| {
            let history = beam_travel(&data, *start).into_iter();
            let mut positions: HashSet<(usize, usize)> = history.map(|x| x.position).collect();
            positions.iter().count()
        }).max().unwrap();


    println!("Result: {:?}", res);
}
