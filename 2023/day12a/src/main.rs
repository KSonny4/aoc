fn count_arrangements(num: i32, row: &str, damaged_groups: &[usize]) -> i32 {
    if damaged_groups.len() == 0 {
        if  row.contains('?') {
            panic!()
        } else {
            return num;
        }
    }

    let mut index = 0;
    let dmg_group_len = damaged_groups[index];
    let group: Vec<char> = (0..dmg_group_len).map(|_| '#').collect();
    let placement: Vec<char> = (0..dmg_group_len).map(|_| '?').collect();



    if let Some(index) = row.find(placement.iter().collect::<String>().as_str()) {
        // Replace the current unknown spring with a broken one
        let mut new_row = row.chars().collect::<Vec<_>>();
        new_row[index..index + dmg_group_len].clone_from_slice(&group);
        let count1 = count_arrangements(num, &new_row.iter().collect::<String>(), &damaged_groups[1..]);

        new_row[index] = '.';
        let count2 = count_arrangements(num, &new_row.iter().collect::<String>(), &damaged_groups[1..]);

        count1 + count2
    } else {
        // All unknown springs are replaced, check if the arrangement is valid
        let row_str = row.to_string();
        if damaged_groups.len() > 0 || row.contains('?') {
            0
        } else {
            (num as usize).try_into().unwrap()
        }
    }
}

fn main() {
    let input = "?###???????? 3,2,1
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    let total_arrangements: i32 = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0];
            let damaged_groups: Vec<usize> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            count_arrangements(index as i32, row, &damaged_groups)
        })
        .sum();

    println!("Total arrangements: {}", total_arrangements);
}
