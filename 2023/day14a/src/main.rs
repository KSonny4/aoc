fn try_move_rounded_rock(line: Vec<char>, rock_to_move: usize) -> Vec<char> {
    let mut index_to_move = None;
    let lin: Vec<char> = line[0..rock_to_move].iter().copied().rev().collect();

    for (index, item) in lin.iter().enumerate() {
        match item {
            '.' => index_to_move = Some(lin.len() - 1 - index),
            'O' => break,
            '#' => break,
            _ => panic!("unreachable"),
        }
    }

    if let Some(index_to_move) = index_to_move {
        let mut line = line.clone();
        line.swap(rock_to_move, index_to_move);
        line
    } else {
        line
    }
}

fn move_stones_in_1d_vec(data: Vec<char>) -> Vec<char> {
    let mut d = data.clone();
    for index in 1..data.len() {
        let element_to_move = d[index];
        match element_to_move {
            'O' => d = try_move_rounded_rock(d, index),
            _ => (),
        }
    }
    d
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

fn main() {
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rotated_data = rotate_rows_to_columns(data.clone());
    let moved_stones = move_stones_in_1d_vec(rotated_data[0].clone());
    println!("original {:?}", rotated_data[0].clone());
    println!("         {:?}", moved_stones);

    let res: Vec<Vec<char>> = rotated_data.iter().map(|x| move_stones_in_1d_vec(x.clone())).collect();
    for line in &res {
        println!("{:?}", line);
    }

    let reversed_res: Vec<Vec<char>> = res.iter().map(|x| x.iter().rev().copied().collect()).collect();
    for line in &reversed_res {
        println!("{:?}", line);
    }

    // replace every 'O' with its index, replace the rest with 0
    let res2: Vec<Vec<usize>> = reversed_res.iter().map(|x| x.iter().enumerate().map(|(index, y) | match y {
        'O' => index + 1,
        _ => 0,
    }).collect()).collect();
    for line in &res2 {
        println!("{:?}", line);
    }

    // sum whole Vec<Vec<i32>> into i32
    let res3: i32 = res2.iter().map(|x| x.iter().sum::<usize>() as i32).sum();
    println!("res3: {}", res3);
}
