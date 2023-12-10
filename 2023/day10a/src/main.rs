use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    value: String,
    loc_x: usize,
    loc_y: usize,
}


fn dfs(start: &Point, input: Vec<Vec<Point>>) -> Option<Vec<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut history: Vec<Point> = Vec::new();
    let mut current_vertex: Point;

    current_vertex = start.clone();
    visited.insert(start.clone());

    loop {
        history.push(current_vertex.clone());

        let current_vertex_conns = connected_to(current_vertex.clone(), &input);
        for neighbor in current_vertex_conns {
            let neigh_vertex_conns = connected_to(neighbor.clone(), &input);
            if neigh_vertex_conns.contains(&current_vertex) {
                if neighbor.value == "S" && history.len() > 2 {
                    history.push(neighbor);
                    return Some(history);
                }

                if visited.insert(neighbor.clone()) {
                    current_vertex = neighbor.clone();
                    break; // no need for other vecs
                }
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<Point>> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(index2, s)| Point {
                    value: s.to_string(),
                    loc_x: index,
                    loc_y: index2,
                })
                .collect()
        })
        .collect();

    // println!("{:?}", input);

    // for i in &input {
    //     let g: String = i.iter().map(|x| x.value.clone()).collect();
    //     println!("{:#?}", g);
    // }


    let start: &Point = input.iter().find_map(|row| {
        row.iter().find_map(|col| {
            if col.value == "S" {
                Some(col)
            } else {
                None
            }
        })
    }).unwrap();


    //let input_with_neigbors = fill_neighbors(input.clone());
    //println!("{:?}", input_with_neigbors);

    // println!("{:?}", start);


    let cycle = dfs(start, input.clone());

    // println!("{:#?}", cycle.clone());
    println!("{:?}", cycle.clone().unwrap().len());
    println!("{:?}", cycle.clone().unwrap().len() / 2);
}


fn connected_to(current_point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    match current_point.value.as_str() {
        "|" => vertical_pipe_conns(&current_point, &input),
        "-" => horizontal_pipe_conns(current_point, &input),
        "L" => l_pipe_conns(current_point, &input),
        "J" => j_pipe_conns(current_point, &input),
        "7" => seven_pipe_conns(current_point, &input),
        "F" => f_pipe_conns(current_point, &input),
        "." => vec![],
        "S" => s_pipe_conns(current_point, &input),
        _ => panic!()
    }
}

fn vertical_pipe_conns(point: &Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for &col_offset in &[1, -1] {
        let new_row_idx = point.loc_x as isize + col_offset;
        let new_col_idx = point.loc_y as isize;

        // Check if the new point is within bounds
        if new_col_idx >= 0 && new_col_idx < input[0].len() as isize {
            let conn = input[new_row_idx as usize][new_col_idx as usize].clone();
            if conn.value != "." {
                v.push(conn);
            }
        }
    }
    v
}


fn s_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let offsets = [
        (-1, 0),
        (0, -1), (0, 1),
        (1, 0),
    ];

    let mut neighbors: Vec<Point> = Vec::new();

    for &(row_offset, col_offset) in &offsets {
        let new_row_idx = point.loc_x as isize + row_offset;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_row_idx >= 0
            && new_row_idx < input.len() as isize
            && new_col_idx >= 0
            && new_col_idx < input[0].len() as isize
        {
            let neighbor = input[new_row_idx as usize][new_col_idx as usize].clone();

            if neighbor.value != "." {
                neighbors.push(neighbor);
            }
        }
    }

    neighbors
}

fn f_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for option in &[&[0, 1], &[1, 0]] {
        let (row_offset, col_offset) = (option[0], option[1]);
        let new_row_idx = point.loc_x as isize + row_offset;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_row_idx >= 0 && new_row_idx < input.len() as isize
            && new_col_idx >= 0 && new_col_idx < input[0].len() as isize
        {
            let conn = &input[new_row_idx as usize][new_col_idx as usize];
            if conn.value != "." {
                v.push(conn.clone());
            }
        }
    }
    v
}

fn seven_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for option in &[&[0, -1], &[1, 0]] {
        let (row_offset, col_offset) = (option[0], option[1]);
        let new_row_idx = point.loc_x as isize + row_offset;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_row_idx >= 0 && new_row_idx < input.len() as isize
            && new_col_idx >= 0 && new_col_idx < input[0].len() as isize
        {
            let conn = &input[new_row_idx as usize][new_col_idx as usize];
            if conn.value != "." {
                v.push(conn.clone());
            }
        }
    }
    v
}

fn j_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for option in &[&[-1, 0], &[0, -1]] {
        let (row_offset, col_offset) = (option[0], option[1]);
        let new_row_idx = point.loc_x as isize + row_offset;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_row_idx >= 0 && new_row_idx < input.len() as isize
            && new_col_idx >= 0 && new_col_idx < input[0].len() as isize
        {
            let conn = &input[new_row_idx as usize][new_col_idx as usize];
            if conn.value != "." {
                v.push(conn.clone());
            }
        }
    }

    v
}


fn l_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for option in &[&[-1, 0], &[0, 1]] {
        let (row_offset, col_offset) = (option[0], option[1]);
        let new_row_idx = point.loc_x as isize + row_offset;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_row_idx >= 0 && new_row_idx < input.len() as isize
            && new_col_idx >= 0 && new_col_idx < input[0].len() as isize
        {
            let conn = &input[new_row_idx as usize][new_col_idx as usize];
            if conn.value != "." {
                v.push(conn.clone());
            }
        }
    }

    v
}


fn horizontal_pipe_conns(point: Point, input: &Vec<Vec<Point>>) -> Vec<Point> {
    let mut v = Vec::new();

    for &col_offset in &[1, -1] {
        let new_row_idx = point.loc_x as isize;
        let new_col_idx = point.loc_y as isize + col_offset;

        // Check if the new point is within bounds
        if new_col_idx >= 0 && new_col_idx < input[0].len() as isize {
            let conn = input[new_row_idx as usize][new_col_idx as usize].clone();
            if conn.value != "." {
                v.push(conn);
            }
        }
    }
    v
}