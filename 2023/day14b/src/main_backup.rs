use std::ops::Range;

fn move_stones_in_1d_vec(line: Vec<char>, direction: Range<usize>) -> Vec<char> {
    line.iter()
        .enumerate()
        .rev()
        .fold(line.clone(), |mut moved_line, (index, &element_to_move)| {
            if element_to_move == 'O' {
                let index_to_move = moved_line[direction.start..index]
                    .iter()
                    .copied()
                    .rev()
                    .position(|item| matches!(item, '.'));

                if let Some(i) = index_to_move {
                    moved_line.swap(index, direction.start + i);
                }
            }
            moved_line
        })
}


fn move_in_direction(data: Vec<Vec<char>>, direction: Range<usize>) -> Vec<Vec<char>> {
    let rotated_data = rotate_rows_to_columns(data);
    let moved_data: Vec<Vec<char>> = rotated_data
        .iter()
        .map(|line| move_stones_in_1d_vec(line.clone(), direction.clone()))
        .collect();
    rotate_rows_to_columns(moved_data)
}

fn rotate_rows_to_columns(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut p1: Vec<Vec<char>> = Vec::new();

    for i in 0..p0[0].len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..p0.len() {
            row.push(p0[j][i].clone());
        }
        p1.push(row);
    }

    p1
}

fn move_north(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    move_in_direction(data, 0..data[0].len())
}

fn move_east(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    move_in_direction(data, 0..data.len())
}

fn move_south(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rotated_data = rotate_rows_to_columns(data);
    let moved_data: Vec<Vec<char>> = rotated_data
        .iter()
        .map(|line| move_stones_in_1d_vec(line.clone(), 1..line.len()))
        .collect();
    rotate_rows_to_columns(moved_data)
}

fn move_west(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    move_in_direction(data, 1..data.len())
}

fn compute_score(p0: Vec<Vec<char>>, cycle: usize, history: Vec<Vec<Vec<char>>>) -> i32 {
    for line in &p0 {
        println!("{:?}", line);
    }

    let index = history.iter().position(|x| x == &p0).unwrap();
    let cycle_loop = history[index..].to_vec();
    let cycles_remaining = 1000000000 - cycle;
    let index_bil = cycles_remaining % cycle_loop.len();
    let p0 = cycle_loop[index_bil].clone();

    let res2: Vec<Vec<usize>> = p0
        .iter()
        .rev()
        .enumerate()
        .map(|(index_x, x)| {
            x.iter()
                .enumerate()
                .map(|(index, y)| match y {
                    'O' => index_x + 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let res3: i32 = res2.iter().map(|x| x.iter().sum::<usize>() as i32).sum();
    println!("res3: {}", res3);

    res3
}

fn get_cycle(data: Vec<Vec<char>>) -> i32 {
    let mut d = data.clone();

    let mut history: Vec<Vec<Vec<char>>> = Vec::new();
    let mut cycle = 1;

    while true {
        let north = move_north(d.clone());
        let west = move_west(north.clone());
        let south = move_south(west.clone());
        d = move_east(south.clone());

        if history.contains(&d) {
            println!("Found a loop! cycle: {}", cycle);
            return compute_score(d.clone(), cycle, history);
        }
        history.push(d.clone());
        cycle += 1;
    }
    panic!("unreachable");
}

fn main() {
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    get_cycle(data.clone());
}
