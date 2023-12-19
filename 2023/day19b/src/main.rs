use std::collections::HashMap;

#[derive(Debug)]
struct Workflow {
    function_name: String,
    condition: Vec<Vec<String>>,
    return_value: String,
}

fn main() {
    let input = include_str!("input.txt");
    let splitted: Vec<&str> = input.split("\n\n").collect();

    let workflows: Vec<Workflow> = splitted[0].split('\n').map(|captures| {
        let gg: Vec<&str> = captures.split("{").collect();
        let function_name = gg[0].to_string();
        let func_bodies = gg[1].replace("}", "");

        let func_bodies_owned: String = func_bodies.clone();

        let func_bodies_split = func_bodies_owned.split(",").collect::<Vec<&str>>();
        let return_value = func_bodies_split[func_bodies_split.len() - 1].to_string();
        let conditions: Vec<Vec<String>> = func_bodies_split[..func_bodies_split.len() - 1]
            .iter()
            .map(|cond| cond.split(":").map(String::from).collect::<Vec<String>>())
            .collect();

        println!("Workflow: {}", captures);
        println!("Conditions: {:?} returns: {}", conditions, return_value);

        Workflow {
            function_name,
            condition: conditions,
            return_value,
        }
    }).collect();

    println!("Workflows: {:?}", workflows);

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflows {
        workflow_map.insert(workflow.function_name.clone(), workflow);
    }

    let mut ranges: HashMap<String, (i64, i64)> = HashMap::new();
    ranges.insert("x".to_string(), (1, 4000));
    ranges.insert("m".to_string(), (1, 4000));
    ranges.insert("a".to_string(), (1, 4000));
    ranges.insert("s".to_string(), (1, 4000));


    let res = get_ranges("in".to_string(), &mut ranges, &workflow_map);
    println!("Result: {}", res);
}

fn get_ranges(current: String, ranges: &mut HashMap<String, (i64, i64)>, workflow_map: &HashMap<String, Workflow>) -> i64 {
    if current == 'R'.to_string() {
        return 0;
    }

    if current == 'A'.to_string() {
        let x: Vec<i64> = ranges.values().map(|&(mmin, mmax)| mmax - mmin + 1).collect();
        return x.iter().product();
    }

    let (conditions, return_value) = (
        workflow_map[&current].condition.clone(),
        workflow_map[&current].return_value.clone()
    );
    let mut total = 0;
    println!("{}: {:?}", current, conditions);
    println!("{}: {:?}", current, ranges);
    for condition in conditions {
        let (var, op, value) = (
            &condition[0].chars().nth(0),
            &condition[0].chars().nth(1), &condition[0][2..]
        );
        //println!("{:?} {:?} {:?}", var.unwrap(), op.unwrap(), value);

        let value = value.parse::<i64>().unwrap();
        let op = op.unwrap();
        let func_name = var.unwrap().to_string();

        if op == '<' {
            let (min, max) = ranges[&func_name];
            if min < value {
                let mut newranges = ranges.clone();
                newranges.insert((*func_name).parse().unwrap(), (min, value - 1));
                total += get_ranges(condition[1].clone(), &mut newranges, workflow_map);
            }

            if max >= value {
                ranges.insert((*func_name).parse().unwrap(), (value, max));
            } else {
                break;
            }
        } else {
            let (min, max) = ranges[&func_name];
            if max > value {
                let mut newranges = ranges.clone();
                newranges.insert((*func_name).parse().unwrap(), (value + 1, max));
                total += get_ranges(condition[1].clone(), &mut newranges, workflow_map);
            }

            if min <= value {
                ranges.insert((*func_name).parse().unwrap(), (min, value));
            } else {
                break;
            }
        }
    }

    println!("{}, {:?}", current, ranges);
    total += get_ranges(return_value, &mut ranges.clone(), workflow_map);
    total
}