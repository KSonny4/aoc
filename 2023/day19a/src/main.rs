use std::collections::HashMap;
use regex::Regex;

use evalexpr::*;

#[derive(Debug, Clone)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}


#[derive(Debug)]
struct Workflow {
    function_name: String,
    condition: Vec<Vec<String>>,
    return_value: String,
}

fn main() {
    // Parse the entire input
    let input = include_str!("input.txt");
    let splitted: Vec<&str> = input.split("\n\n").collect();

    let workflows: Vec<Workflow> = splitted[0].split('\n').map(|captures| {
        let gg: Vec<&str> = captures.split("{").collect();
        let function_name = gg[0].to_string();
        let func_bodies = gg[1].replace("}", "");

        // Move func_bodies into a new String, avoiding borrowing issues
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

    let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let parts = splitted[1].split('\n').map(|captures| {
        if let Some(captures) = re.captures(captures) {
            let x: i32 = captures[1].parse().unwrap();
            let m: i32 = captures[2].parse().unwrap();
            let a: i32 = captures[3].parse().unwrap();
            let s: i32 = captures[4].parse().unwrap();
            Part {
                x,
                m,
                a,
                s,
            }
        } else {
            panic!("Not parsed: {}", captures)
        }
    }).collect::<Vec<Part>>();

    println!("Parts: {:?}", parts);

    get_accepted_parts(&parts, &workflow_map);
}


fn evaluate_workflow(workflow: &Workflow, part: &Part) -> String {
    let context = context_map! {
    "x" => Value::from(part.x as f64),
    "m" => Value::from(part.m as f64),
    "a" => Value::from(part.a as f64),
    "s" => Value::from(part.s as f64),
}.unwrap();
    println!("workflow: {:?}, part: {:?}", workflow, part);
    for condition in &workflow.condition {
        let (cond, result) = condition.split_at(1);
        let precompiled = build_operator_tree(cond[0].as_str()).unwrap();
        let res = precompiled.eval_with_context(&context);
        if res.is_ok_and(|x| x == Value::from(true)) {
            println!("Condition: {:?} is true", condition);
            return result[0].clone();
        }
    }
    println!("No condition is true, returning: {}", workflow.return_value);
    workflow.return_value.to_string()
}

fn get_part_state(part: &Part, workflows: &HashMap<String, Workflow>, starting_workflow: &Workflow) -> (Part, String) {
    let mut workflow = starting_workflow.clone();
    while let result = evaluate_workflow(&workflow, part) {
        if result == "A" || result == "R" {
            return (part.clone(), result);
        }
        workflow = workflows.get(&*result).unwrap().clone();
    }
    panic!("No workflow found for part: {:?}", part)
}
fn get_accepted_parts(parts: &Vec<Part>, workflows: &HashMap<String, Workflow>) {
    let mut accepted_parts: Vec<(Part, String)> = Vec::new();
    let starting_workflow = workflows.get("in").unwrap();
    for part in parts {
        println!("====================");
        println!("Part: {:?}", part);
        accepted_parts.push(get_part_state(part, workflows, starting_workflow));

        println!("====================");
    }
    let x: Vec<(Part, String)> = accepted_parts
        .iter()
        .filter(|(_, status)| status == "A")
        .cloned()  // Add this line to clone the values
        .collect();


    let res: i32 = x.iter().map(|(part, _)| part.x + part.a + part.m + part.s).sum();
    println!("Result: {}", res);
}