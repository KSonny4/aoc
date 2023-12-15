// use std::collections::HashSet;
//
// fn try_move_rounded_rock_north(line: Vec<char>, rock_to_move: usize) -> Vec<char> {
//     let mut index_to_move = None;
//     let lin: Vec<char> = line[0..rock_to_move].iter().copied().rev().collect();
//
//     for (index, item) in lin.iter().enumerate() {
//         match item {
//             '.' => index_to_move = Some(lin.len() - 1 - index),
//             'O' => break,
//             '#' => break,
//             _ => panic!("unreachable"),
//         }
//     }
//
//     if let Some(index_to_move) = index_to_move {
//         let mut line = line.clone();
//         line.swap(rock_to_move, index_to_move);
//         line
//     } else {
//         line
//     }
// }
//
// fn move_stones_in_1d_vec_north(data: Vec<char>) -> Vec<char> {
//     let mut d = data.clone();
//     for index in 1..data.len() {
//         let element_to_move = d[index];
//         match element_to_move {
//             'O' => d = try_move_rounded_rock_north(d, index),
//             _ => (),
//         }
//     }
//     d
// }
//
// fn rotate_rows_to_columns(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let mut p1: Vec<Vec<char>> = Vec::new();
//
//     for i in 0..p0[0].len() {
//         let mut row: Vec<char> = Vec::new();
//         for j in 0..p0.len() {
//             row.push(p0[j][i].clone());
//         }
//         p1.push(row);
//     }
//
//     p1
// }
//
// fn move_east(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     p0.iter().map(|line|
//         move_stones_in_1d_vec_east(line.clone())
//     ).collect(
//
//     )
// }
//
// fn move_stones_in_1d_vec_east(p0: Vec<char>) -> Vec<char> {
//     let mut p1 = p0.clone();
//     for index in (0..p0.len()).rev() {
//         let element_to_move = p1[index];
//         match element_to_move {
//             'O' => p1 = try_move_rounded_rock_east(p1, index),
//             _ => (),
//         }
//     }
//     p1
// }
//
// fn try_move_rounded_rock_east(p0: Vec<char>, p1: usize) -> Vec<char> {
//     let mut index_to_move = None;
//     let lin: Vec<char> = p0[p1+1..p0.len()].to_vec();
//
//     for (index, item) in lin.iter().enumerate() {
//         match item {
//             '.' => index_to_move = Some(p1 + 1 + index),
//             'O' => break,
//             '#' => break,
//             _ => panic!("unreachable"),
//         }
//     }
//
//     if let Some(index_to_move) = index_to_move {
//         let mut p0 = p0.clone();
//         p0.swap(p1, index_to_move);
//         p0
//     } else {
//         p0
//     }
// }
//
// fn move_south(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let rotated_data = rotate_rows_to_columns(p0);
//     let res: Vec<Vec<char>> = rotated_data.iter().map(|x| move_stones_in_1d_vec_south(x.clone())).collect();
//     rotate_rows_to_columns(res).iter().rev().cloned().collect()
// }
//
// fn move_stones_in_1d_vec_south(p0: Vec<char>) -> Vec<char> {
//     let mut p1: Vec<char> = p0.iter().copied().rev().collect();
//     for index in 1..p0.len() {
//         let element_to_move = p1[index];
//         match element_to_move {
//             'O' => p1 = try_move_rounded_rock_south(p1, index),
//             _ => (),
//         }
//     }
//     p1
// }
//
// fn try_move_rounded_rock_south(p0: Vec<char>, p1: usize) -> Vec<char> {
//     let mut index_to_move = None;
//     let lin: Vec<char> = p0[0..p1].iter().copied().rev().collect();
//
//     for (index, item) in lin.iter().enumerate() {
//         match item {
//             '.' => index_to_move = Some(lin.len() - 1 - index),
//             'O' => break,
//             '#' => break,
//             _ => panic!("unreachable"),
//         }
//     }
//
//     if let Some(index_to_move) = index_to_move {
//         let mut p0 = p0.clone();
//         p0.swap(p1, index_to_move);
//         p0
//     } else {
//         p0
//     }
// }
//
// fn move_north(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     let rotated_data = rotate_rows_to_columns(data);
//     let res: Vec<Vec<char>> = rotated_data.iter().map(|x| move_stones_in_1d_vec_north(x.clone())).collect();
//     rotate_rows_to_columns(res)
// }
//
//
// fn move_west(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
//     data.iter().map(|line|
//         move_stones_in_1d_vec_west(line.clone())
//     ).collect(
//
//     )
// }
//
// fn move_stones_in_1d_vec_west(p0: Vec<char>) -> Vec<char> {
//     let mut p1 = p0.clone();
//     for index in 1..p0.len() {
//         let element_to_move = p1[index];
//         match element_to_move {
//             'O' => p1 = try_move_rounded_rock_west(p1, index),
//             _ => (),
//         }
//     }
//     p1
// }
//
// fn try_move_rounded_rock_west(p0: Vec<char>, p1: usize) -> Vec<char> {
//     let mut index_to_move = None;
//     let lin: Vec<char> = p0[0..p1].iter().copied().rev().collect();
//
//     for (index, item) in lin.iter().enumerate() {
//         match item {
//             '.' => index_to_move = Some(lin.len() - 1 - index),
//             'O' => break,
//             '#' => break,
//             _ => panic!("unreachable"),
//         }
//     }
//
//     if let Some(index_to_move) = index_to_move {
//         let mut p0 = p0.clone();
//         p0.swap(p1, index_to_move);
//         p0
//     } else {
//         p0
//     }
// }
//
//
//
//
//
// fn main() {
//     let data: Vec<Vec<char>> = include_str!("input_simple.txt")
//         .lines()
//         .map(|line| line.chars().collect())
//         .collect();
//
//     for line in &data {
//         println!("{:?}", line);
//     }
//     println!("  ");
//
//
//     let res = get_cycle(data.clone());
//
//     let reversed_res: Vec<Vec<char>> = rotate_rows_to_columns(res);
//     // replace every 'O' with its index, replace the rest with 0
//     let res2: Vec<Vec<usize>> = reversed_res.iter().map(|x| x.iter().enumerate().map(|(index, y) | match y {
//         'O' => index + 1,
//         _ => 0,
//     }).collect()).collect();
//
//     // sum whole Vec<Vec<i32>> into i32
//     let res3: i32 = res2.iter().map(|x| x.iter().sum::<usize>() as i32).sum();
//     println!("res3: {}", res3);
//
//     // let res3: Vec<Vec<usize>> = res.iter().map(|x| x.iter().enumerate().map(|(index, y) | match y {
//     //     'O' => index + 1,
//     //     _ => 0,
//     // }).collect()).collect();
//     //
//     // // sum whole Vec<Vec<i32>> into i32
//     // let res4: i32 = res3.iter().map(|x| x.iter().sum::<usize>() as i32).sum();
//     // println!("res4: {}", res4);
//
// }
//
// fn get_cycle(data: Vec<Vec<char>>)  -> Vec<Vec<char>>{
//
//     let mut d = data.clone();
//
//     let mut history :HashSet<Vec<Vec<char>>> = HashSet::new();
//     let mut cycle = 1;
//
//     while true {
//         let north = move_north(d.clone());
//         let west = move_west(north.clone());
//         let south = move_south(west.clone());
//         d = move_east(south.clone());
//
//         if history.contains(&d) {
//             println!("Found a loop! cycle: {}", cycle);
//             return d.clone();
//             // return north; // TODO do not know if this is correct
//         }
//
//         history.insert(d.clone());
//
//         println!("Attempt: {}", cycle);
//         for line in &d {
//             println!("{:?}", line);
//         }
//         println!("  ");
//
//         cycle += 1;
//     }
//     panic!("unreachable");
// }


use std::collections::HashSet;

fn try_move_rounded_rock_north(line: Vec<char>, rock_to_move: usize) -> Vec<char> {
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

fn move_stones_in_1d_vec_north(data: Vec<char>) -> Vec<char> {
    let mut d = data.clone();
    for index in 1..data.len() {
        let element_to_move = d[index];
        match element_to_move {
            'O' => d = try_move_rounded_rock_north(d, index),
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

fn move_east(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
    p0.iter().map(|line|
        move_stones_in_1d_vec_east(line.clone())
    ).collect(

    )
}

fn move_stones_in_1d_vec_east(p0: Vec<char>) -> Vec<char> {
    let mut p1 = p0.clone();
    for index in (0..p0.len()).rev() {
        let element_to_move = p1[index];
        match element_to_move {
            'O' => p1 = try_move_rounded_rock_east(p1, index),
            _ => (),
        }
    }
    p1
}

fn try_move_rounded_rock_east(p0: Vec<char>, p1: usize) -> Vec<char> {
    let mut index_to_move = None;
    let lin: Vec<char> = p0[p1+1..p0.len()].to_vec();

    for (index, item) in lin.iter().enumerate() {
        match item {
            '.' => index_to_move = Some(p1 + 1 + index),
            'O' => break,
            '#' => break,
            _ => panic!("unreachable"),
        }
    }

    if let Some(index_to_move) = index_to_move {
        let mut p0 = p0.clone();
        p0.swap(p1, index_to_move);
        p0
    } else {
        p0
    }
}

fn move_south(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rotated_data = rotate_rows_to_columns(p0);
    let res: Vec<Vec<char>> = rotated_data.iter().map(|x| move_stones_in_1d_vec_south(x.clone())).collect();
    rotate_rows_to_columns(res).iter().rev().cloned().collect()
}

fn move_stones_in_1d_vec_south(p0: Vec<char>) -> Vec<char> {
    let mut p1: Vec<char> = p0.iter().copied().rev().collect();
    for index in 1..p0.len() {
        let element_to_move = p1[index];
        match element_to_move {
            'O' => p1 = try_move_rounded_rock_south(p1, index),
            _ => (),
        }
    }
    p1
}

fn try_move_rounded_rock_south(p0: Vec<char>, p1: usize) -> Vec<char> {
    let mut index_to_move = None;
    let lin: Vec<char> = p0[0..p1].iter().copied().rev().collect();

    for (index, item) in lin.iter().enumerate() {
        match item {
            '.' => index_to_move = Some(lin.len() - 1 - index),
            'O' => break,
            '#' => break,
            _ => panic!("unreachable"),
        }
    }

    if let Some(index_to_move) = index_to_move {
        let mut p0 = p0.clone();
        p0.swap(p1, index_to_move);
        p0
    } else {
        p0
    }
}

fn move_north(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rotated_data = rotate_rows_to_columns(data);
    let res: Vec<Vec<char>> = rotated_data.iter().map(|x| move_stones_in_1d_vec_north(x.clone())).collect();
    rotate_rows_to_columns(res)
}


fn move_west(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    data.iter().map(|line|
        move_stones_in_1d_vec_west(line.clone())
    ).collect(

    )
}

fn move_stones_in_1d_vec_west(p0: Vec<char>) -> Vec<char> {
    let mut p1 = p0.clone();
    for index in 1..p0.len() {
        let element_to_move = p1[index];
        match element_to_move {
            'O' => p1 = try_move_rounded_rock_west(p1, index),
            _ => (),
        }
    }
    p1
}

fn try_move_rounded_rock_west(p0: Vec<char>, p1: usize) -> Vec<char> {
    let mut index_to_move = None;
    let lin: Vec<char> = p0[0..p1].iter().copied().rev().collect();

    for (index, item) in lin.iter().enumerate() {
        match item {
            '.' => index_to_move = Some(lin.len() - 1 - index),
            'O' => break,
            '#' => break,
            _ => panic!("unreachable"),
        }
    }

    if let Some(index_to_move) = index_to_move {
        let mut p0 = p0.clone();
        p0.swap(p1, index_to_move);
        p0
    } else {
        p0
    }
}





fn main() {
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    //
    // for line in &data {
    //     println!("{:?}", line);
    // }
    // println!("  ");


    get_cycle(data.clone());

}

fn compute_score(p0: Vec<Vec<char>>, cycle: usize, history: Vec<Vec<Vec<char>>> ) -> i32 {
    for line in &p0 {
        println!("{:?}", line);
    }

    let index = history.iter().position(|x| x == &p0).unwrap();
    let cycle_loop = history[index..].to_vec();
    let cycles_remaining = 1000000000 - cycle;
    let index_bil = cycles_remaining % cycle_loop.len();
    let p0 = cycle_loop[index_bil].clone();


    //let reversed_res: Vec<Vec<char>> = rotate_rows_to_columns(p0);
    // replace every 'O' with its index, replace the rest with 0
    let res2: Vec<Vec<usize>> = p0.iter().rev().enumerate().map(|(index_x, x) | x.iter().enumerate().map(|(index, y) | match y {
        'O' => index_x + 1,
        _ => 0,
    }).collect()).collect();

    // sum whole Vec<Vec<i32>> into i32
    let res3: i32 = res2.iter().map(|x| x.iter().sum::<usize>() as i32).sum();
    println!("res3: {}", res3);

    res3
}

fn get_cycle(data: Vec<Vec<char>>) ->i32 {

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
