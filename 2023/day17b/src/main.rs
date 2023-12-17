use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

fn calculate_shortest_path(input: &str, min_moves: usize, max_moves: usize) -> u32 {
    let grid = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10))),
    ).unwrap();

    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(position, (delta_row, delta_col), level)| {
            let mut next_steps = Vec::with_capacity(3);
            let mut explore_direction = |direction, level| {
                next_steps.extend(
                    &grid
                        .move_in_direction(position, direction)
                        .map(|target| ((target, direction, level), grid[target])),
                );
            };

            if level < max_moves {
                explore_direction((delta_row, delta_col), level + 1);
            }

            if level >= min_moves {
                explore_direction((-delta_col, -delta_row), 1);
                explore_direction((delta_col, delta_row), 1);
            } else if level == 0 {
                explore_direction((1, 0), 1);
                explore_direction((0, 1), 1);
            }

            next_steps
        },
        |&(position, _, level)| position == (grid.rows - 1, grid.columns - 1) && level >= min_moves,
    ).unwrap().1
}

fn main() {
    let input_content = include_str!("input.txt");

    let result_min_1_max_3 = calculate_shortest_path(input_content, 1, 3);
    let result_min_4_max_10 = calculate_shortest_path(input_content, 4, 10);

    println!("{:?}", result_min_1_max_3);
    println!("{:?}", result_min_4_max_10);
}
