
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn is_valid(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as isize && y >= 0 && y < cols as isize
}

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    ((p1.x as isize - p2.x as isize).abs() + (p1.y as isize - p2.y as isize).abs()) as usize
}
fn find_shortest_paths(grid: &Vec<Vec<char>>, targets: &Vec<Point>) -> HashMap<(Point, Point), usize> {
    let mut distances: HashMap<(Point, Point), usize> = HashMap::new();

    for &start in targets {
        for &target in targets {
            if start != target && !distances.contains_key(&(target,start)){

                    distances.insert((start, target), manhattan_distance(start, target));

            }
        }
    }

    distances
}

fn main() {


    let mut input: Vec<Vec<char>> = include_str!("input_simple.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut i = 0;
    // add rows
    while i < input.len() {
        if input[i].iter().all(|y| *y == '.') {
            input.insert(i, vec!['.'; input[0].len()]);
            i += 1; // skip added input
        }
        i += 1;
    }

    let mut i = 0;
    let mut row_len = input[0].len(); // Assuming all rows have the same length

// add columns
    while i < row_len {
        if input.iter().all(|row| row[i] == '.') {
            for row in &mut input {
                row.insert(i, '.');
            }
            i += 1; // skip added column
            row_len += 1;
        }
        i += 1;
    }

    // for x in &input {
    //     for y in x {
    //         print!("{} ", y);
    //     }
    //     println!();
    // }

    let mut targets: Vec<Point> = Vec::new();

    let mut mapping: HashMap<Point, usize> = HashMap::new();

    // Find positions of all '#'
    let mut name = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                targets.push(Point {  x: i, y: j });
                name += 1;
                mapping.insert(Point {  x: i, y: j }, name);
            }
        }
    }

    println!("{}", targets.len());

    let distances = find_shortest_paths(&input, &targets);


    for (&(start, target), &distance) in &distances {
        println!("Distance between {:?} and {:?}: {}", mapping[&start], mapping[&target], distance);
    }

    println!("{:?}", distances.len());


    println!("{:?}", distances.iter().map(|x| x.1).sum::<usize>());

    // let res: usize = distances.iter().map(|(_, distance) | (distance*2) + 1).sum();
    // println!("{:?}", res);





}

 // . . . . # . . . . . . . .
// . . . . . . . . . # . . .
// # . . . . . . . . . . . .
// . . . . . . . . . . . . .
// . . . . . . . . . . . . .
// . . . . . . . . # . . . .
// . # . . . . . . . . . . .
// . . . . . . . . . . . . #
// . . . . . . . . . . . . .
// . . . . . . . . . . . . .
// . . . . . . . . . # . . .
// # . . . . # . . . . . . .