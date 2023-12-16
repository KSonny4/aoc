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

    println!("{:?}", data);
    let history = beam_travel(&data, BeamPoint { position: (0, 0), came_from_direction: Direction::Right, symbol: data[0][0] });
    for (i, beampoint) in data.iter().enumerate() {
        for (j, symbol) in beampoint.iter().enumerate() {
            let position_in_res = history.iter().position(|x| x.position == (i, j));
            if let Some(_) = position_in_res {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    // create hashset of (i, j) positions
    let mut positions: HashSet<(usize, usize)> = history.clone().into_iter().map(|x| x.position).collect();
    println!("Count: {}", positions.iter().count());
}
