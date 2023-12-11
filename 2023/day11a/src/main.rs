
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn is_valid(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as isize && y >= 0 && y < cols as isize
}

fn bfs_distance(grid: &Vec<Vec<char>>, start: Point, target: Point) -> Option<usize> {
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, distance)) = queue.pop_front() {
        if current == target {
            return Some(distance);
        }

        for (dx, dy) in &directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if is_valid(nx, ny, rows, cols) {
                let neighbor = Point {
                    x: nx as usize,
                    y: ny as usize,
                };

                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }
    }

    None
}

fn find_shortest_paths(grid: &Vec<Vec<char>>, targets: &Vec<Point>) -> HashMap<(Point, Point), usize> {
    let mut distances: HashMap<(Point, Point), usize> = HashMap::new();
    let mut count = 0;
    for &start in targets {
        for &target in targets {
            if start != target && !distances.contains_key(&(target,start)){
                if let Some(distance) = bfs_distance(grid, start, target) {
                    count +=1 ;
                    distances.insert((start, target), distance);
                }
            }
        }
    }

    distances
}



//
// fn find_shortest_paths(grid: &Vec<Vec<char>>, targets: &Vec<Point>) -> HashMap<(Point, Point), usize> {
//     let mut distances: HashMap<(Point, Point), usize> = HashMap::new();
//
//     targets.par_iter().for_each(|&start| {
//         targets.iter().for_each(|&target| {
//             if start != target && !distances.contains_key(&(target, start)) {
//                 if let Some(distance) = bfs_distance(grid, start, target) {
//                     distances.insert((start, target), distance);
//                 }
//             }
//         });
//     });
//
//     distances
// }

fn main() {


    let mut input: Vec<Vec<char>> = include_str!("input.txt")
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

    println!("{:?}", targets.len());

    let distances = find_shortest_paths(&input, &targets);

 // Print distances
 //    for (&(start, target), &distance) in &distances {
 //        println!("Distance between {:?} and {:?}: {}", mapping[&start], mapping[&target], distance);
 //    }
 //
 //    println!("{:?}", distances.len());


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