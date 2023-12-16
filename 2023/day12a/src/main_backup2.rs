
fn replace_first(original: &str, find: &str, replace_with: &str) -> String {
    if let Some(pos) = original.find(find) {
        let (before, after) = original.split_at(pos);
        let replaced = format!("{}{}{}", before, replace_with, &after[find.len()..]);
        return replaced;
    }
    original.to_string()
}



fn count_arrangements(num: i32, row: &str, damaged_groups: &[usize], original: &str) -> i32 {
    if damaged_groups.len() == 0 {
        if  row.contains('?') {
            return 0;
        } else {
            return num;
        }
    }

    let mut index = 0;
    let dmg_group_len = damaged_groups[index];
    let group: Vec<char> = (0..dmg_group_len).map(|_| '#').collect();
    let placement: Vec<char> = (0..dmg_group_len).map(|_| '?').collect();


    if let Some(index) = row.find(placement.iter().collect::<String>().as_str()) {
        let mut count3 = 0;
        // check that index -1 and index + 1 are either "." or "?"
        let mut new_row: Vec<char> = row.chars().collect();
        if index as i32 - 1 > 0 {
            if new_row[index - 1] == '#' {
                new_row[index] = '.';
                count3 = count_arrangements(num, &new_row.iter().collect::<String>(), &damaged_groups, original);;
            }
            if  new_row[index - 1] != '?' {
                new_row[index - 1] = '.';
            }

        }
        if index + 1 < row.len() - 1 {
            if new_row[index + 1] == '#' {
                return 0;
            }
            if  new_row[index + 1] != '?' {
                new_row[index + 1] = '.';
            }

        }

        // replace the damaged group with the new group
        new_row.splice(index..index + dmg_group_len, group.iter().cloned());
        let count1 = count_arrangements(num+1, &new_row.iter().collect::<String>(), &damaged_groups[1..], original);

        // clone row and replace first ? in it with ., we did not place anything and continue
        let replaced_str = replace_first(row.clone(), "?", ".");
        let count2 = count_arrangements(num, &replaced_str, &damaged_groups, original);

        count1 + count2 + count3
    } else {

        let mut r = row.to_string();
        // All unknown springs are replaced, check if the arrangement is valid
        if damaged_groups.len() > 0 {
            // check that the remaining damaged groups are placed
            damaged_groups.iter().for_each(
                |x| {
                    if r.contains(&"#".repeat(*x)) {
                        r = r.replace(&"#".repeat(*x), "");
                    }
                }
            )

        }
        if r.contains('?') || r.contains('#') {
            0
        }
        else {
            (num as usize).try_into().unwrap()
        }
    }
}

fn main() {
//     let input = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    // let input = "????.#...#... 4,1,1";
    //let input = "?###???????? 3,2,1";

    let input = "???.### 1,1,3";
    let total_arrangements: Vec<i32> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let row = parts[0];
            let damaged_groups: Vec<usize> = parts[1]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            count_arrangements(index as i32, row, &damaged_groups, row)
        })
        .collect();

    println!("Total arrangements: {:?}", total_arrangements);

    println!("Total arrangements: {}", total_arrangements.iter().sum::<i32>());
}
