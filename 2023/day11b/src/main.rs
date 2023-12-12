fn main() {
    let asteroid_map = include_bytes!("input.txt");
    let map_size = asteroid_map.iter().position(|&byte| byte == b'\n').unwrap();
    let (x_coordinates, y_coordinates) = count_coordinates(asteroid_map, map_size);

    let total_manhattan_distance = total_distance(&x_coordinates) + total_distance(&y_coordinates);
    println!("{}", total_manhattan_distance);
}

fn count_coordinates(asteroid_map: &[u8], map_size: usize) -> (Vec<usize>, Vec<usize>) {
    let (x_coordinates, y_coordinates): (Vec<usize>, Vec<usize>) = asteroid_map.iter()
        .enumerate()
        .filter(|&(_, &byte)| byte == b'#')
        .fold((vec![0; map_size], vec![0; map_size]), |(mut x_coordinates, mut y_coordinates), (position, _)| {
            x_coordinates[position % (map_size + 1)] += 1;
            y_coordinates[position / (map_size + 1)] += 1;
            (x_coordinates, y_coordinates)
        });

    (x_coordinates, y_coordinates)
}

fn total_distance(counts: &[usize]) -> usize {
    let (gaps, sum, items, distance) = counts.iter().enumerate().fold((0, 0, 0, 0), |(gaps, sum, items, distance), (index, &count)| {
        let expanded = index + 999_999 * gaps;
        let new_sum = sum + count * expanded;
        let new_items = items + count;
        let new_distance = distance + count * (items * expanded - sum);

        (if count > 0 { gaps } else { gaps + 1 }, new_sum, new_items, new_distance)
    });

    distance
}
