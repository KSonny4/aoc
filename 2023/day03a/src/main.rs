#[derive(Debug, Clone)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
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

    let valid_numbers = numbers.iter().filter(|number_list| number_list.iter().any(
        |number| has_no_adj_symbol(number, data.clone())
    ));

    let concatenated_numbers: Vec<i32> = valid_numbers
        .map(|numbers| numbers.iter().map(|number| number.number.to_string()).collect())
        .map(|number: String| number.parse::<i32>().unwrap_or(0))
        .collect();

    println!("{:?}", concatenated_numbers);

    let s: i32 = concatenated_numbers.iter().sum();
    println!("{:?}", s);
}

fn has_no_adj_symbol(number: &NumberLocation, data: Vec<Vec<char>>) -> bool {
    let max_row = data.len();
    let max_col = data[0].len();

    let row = number.locations.x as usize;
    let col = number.locations.y as usize;




    // this could be simplified <3 :D


    let left = col > 0 && data[row][col - 1] != '.' && !data[row][col - 1].is_digit(10);
    let right = col < max_col - 1 && data[row][col + 1] != '.' && !data[row][col + 1].is_digit(10) ;
    let up = row > 0 && data[row - 1][col] != '.'  && !data[row - 1][col].is_digit(10);
    let down = row < max_row - 1 && data[row + 1][col] != '.' && !data[row + 1][col].is_digit(10) ;


    let upper_left = row > 0 && col > 0 && data[row - 1][col - 1] != '.' && !data[row - 1][col - 1].is_digit(10);
    let upper_right = row > 0 && col < max_col - 1 && data[row - 1][col + 1] != '.' && !data[row - 1][col + 1].is_digit(10);
    let lower_left = row < max_row - 1 && col > 0 && data[row + 1][col - 1] != '.' && !data[row + 1][col - 1].is_digit(10);
    let lower_right = row < max_row - 1 && col < max_col - 1 && data[row + 1][col + 1] != '.' && !data[row + 1][col + 1].is_digit(10);

    left || right || up || down || upper_left || upper_right || lower_left || lower_right

}
