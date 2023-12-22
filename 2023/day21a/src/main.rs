use std::collections::{HashMap, HashSet};

fn main() {
    // Example usage:
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut garden:HashMap<(i32,i32), char> = HashMap::new();
    let steps = 64;
    println!("Steps: {}", steps);

    // find position in grid where symbol is 'S'
    let mut start = (0, 0);
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &c)| {
            garden.insert((i as i32, j as i32), c);
            if c == 'S' {
                start = (i as i32, j as i32);
            }
        })
    });

    let fields = take_steps(&garden, start, steps, grid.len() as i32, grid[0].len() as i32);


    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if fields.contains(&(i as i32, j as i32)) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }


    println!("Fields: {}", fields.len());
}


fn take_steps(garden: &HashMap<(i32, i32), char>, start: (i32, i32), steps: i32,  max_x: i32, max_y: i32) -> HashSet<(i32, i32)> {
    let mut stepped_to = HashSet::new();
    stepped_to.insert(start);

    for _ in 0..steps {
        stepped_to = take_a_step(garden, &stepped_to, max_x, max_y);
    }

    stepped_to
}

fn take_a_step(garden: &HashMap<(i32, i32), char>, from_steps: &HashSet<(i32, i32)>, max_x: i32, max_y: i32) -> HashSet<(i32, i32)> {
    let mut to_steps = HashSet::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for &pos in from_steps {
        for direction in directions {
            let new_x = &pos.0 + &direction.0;
            let new_y = &pos.1 + &direction.1;
            let new_position = (new_x, new_y);
            
            if  !(new_x < max_x && new_y < max_y && new_x >= 0 && new_y >= 0) {
                continue;
            }

            if !garden.contains_key(&new_position) {
                continue;
            }
            if garden[&new_position] == '#' {
                continue;
            }
            to_steps.insert(new_position);
        }
    }
    to_steps
}