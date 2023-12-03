use std::iter::zip;
use rayon::prelude::*;
use std::sync::{Mutex, Arc};
use std::time::Instant; // Add this import

#[derive(Debug, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct NumberLocation {
    number: char,
    locations: Location,
}

fn push_number(numbers: &mut Vec<Vec<NumberLocation>>, number: &mut Vec<NumberLocation>) {
    if !number.is_empty() {
        numbers.push(number.clone());
        number.clear();
    }
}

fn main() {
    // this is the most ugly code i have ever written :D
    // optimize: there is no point to check all the numbers against one "*"
    // optimize: a lot of for loops
    // OR! rewrite the whole stuff :D as there are better ways to get the result :D
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut numbers: Vec<Vec<NumberLocation>> = Vec::new();
    for (index_row, row) in data.iter().enumerate() {
        let mut number: Vec<NumberLocation> = Vec::new();
        for (index_col, &ch) in row.iter().enumerate() {
            match ch {
                _ if ch.is_digit(10) => number.push(NumberLocation { number: ch, locations: Location { x: index_row as i32, y: index_col as i32 } }),
                _ => push_number(&mut numbers, &mut number),
            }
        }
        push_number(&mut numbers, &mut number);
    }

    let start_time = Instant::now(); // Record start time

    let gears = Arc::new(Mutex::new(Vec::new()));

    data.par_iter().enumerate().for_each(|(index_row, row)| {
        row.par_iter().enumerate().for_each(|(index_col, &ch)| {
            if ch == '*' {
                let gears = Arc::clone(&gears);
                gears.lock().unwrap().push(check_gears(index_row, index_col, &numbers));
            }
        });
    });

    // Extract the Vec<i32> from the Mutex after all threads finish
    let gears = Arc::try_unwrap(gears).unwrap().into_inner().unwrap();

    let stop_time = Instant::now(); // Record stop time
    let elapsed_time = stop_time - start_time; // Calculate elapsed time

    println!("{:?}", gears);
    let s: i32 = gears.iter().sum();
    println!("{:?}", s);
    println!("Elapsed time: {:?}", elapsed_time);
}
fn check_gears(index_row: usize, index_col: usize, numbers: &Vec<Vec<NumberLocation>>) -> i32 {
    // too lazy to write this optimized
    println!("{},{}", index_row, index_col);

    for n in numbers {
        for m in numbers {
            if n != m{
                for num1 in n {
                    for num2 in m {
                        if connected_through_gear(&num1, &num2, index_row, index_col){
                            println!("Checking if {:?} and {:?} connected", &num1, &num2);
                            return gear_ratio(n,m);
                        }
                    }
                }
            }
        }
    }
    0
}

fn gear_ratio(p0: &Vec<NumberLocation>, p1: &Vec<NumberLocation>) -> i32 {

    let valid_numbers = vec![p0,p1];
    let concatenated_numbers: Vec<i32> = valid_numbers.into_iter()
        .map(|numbers| numbers.iter().map(|number| number.number.to_string()).collect())
        .map(|number: String| number.parse::<i32>().unwrap_or(0))
        .collect();
    concatenated_numbers[0] * concatenated_numbers[1]
}

fn connected_through_gear(num1: &NumberLocation, num2: &NumberLocation, index_row: usize, index_col: usize) -> bool {
    let a: Vec<i32> = vec![0,0,-1,1,-1,-1,1,1];
    let b: Vec<i32> = vec![-1,1,0,0,-1,1,-1,1];

    let ir = index_row as i32;
    let ic = index_col as i32;

    let mut found1 = false;
    for (i,j) in zip(&a,&b) {
        let x = ir + i;
        let y = ic + j;

        if x == num1.locations.x && y == num1.locations.y  {
            found1 = true;
            break;
        }
    }

    let mut found2 = false;
    for (i,j) in zip(&a,&b) {
        let x = ir + i;
        let y = ic + j;

        if x == num2.locations.x && y == num2.locations.y  {
            found2 = true;
            break;
        }
    }

    found1 && found2
}