use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(String),
    Conjunction(String, HashSet<String>),
    Broadcaster(String, HashSet<String>),
    Button(String, String),
}

impl Module {
    fn new(module_str: &str) -> Self {
        let parts: Vec<&str> = module_str.split_whitespace().collect();
        let name = parts[0].to_string();

        match parts[1].chars().next().unwrap() {
            '%' => Module::FlipFlop(name),
            '&' => {
                let inputs: HashSet<String> = parts[3..].iter().map(|s| s.to_string()).collect();
                Module::Conjunction(name, inputs)
            }
            _ => {
                let outputs: HashSet<String> = parts[3..].iter().map(|s| s.to_string()).collect();
                Module::Broadcaster(name, outputs)
            }
        }
    }

    fn process_pulse(&self, pulses: &mut HashMap<String, Pulse>) -> Pulse {
        match self {
            Module::FlipFlop(name) => match pulses.get(name) {
                Some(&Pulse::Low) => {
                    pulses.insert(name.clone(), Pulse::High);
                    Pulse::High
                }
                Some(&Pulse::High) => {
                    pulses.insert(name.clone(), Pulse::Low);
                    Pulse::Low
                }
                None => {
                    pulses.insert(name.clone(), Pulse::High);
                    Pulse::High
                }
            },
            Module::Conjunction(name, inputs) => {
                let input_pulses: Vec<Pulse> = inputs
                    .iter()
                    .map(|input| pulses.get(input).cloned().unwrap_or(Pulse::Low))
                    .collect();

                let output_pulse = if input_pulses.iter().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                pulses.insert(name.clone(), output_pulse);
                output_pulse
            }
            Module::Broadcaster(_, outputs) => {
                let pulse = pulses
                    .iter()
                    .find_map(|(k, &v)| if outputs.contains(k) { Some(v) } else { None })
                    .unwrap_or(Pulse::Low);

                outputs.iter().for_each(|output| {
                    pulses.insert(output.clone(), pulse.clone());
                });

                pulse
            }
            Module::Button(_, output) => {
                pulses.insert(output.clone(), Pulse::Low);
                Pulse::Low
            }
        }
    }
}

fn main() {
    // Your module configuration input
    let input = "
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a
    ";

    // Parse the input to create module instances
    let modules: Vec<Module> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Module::new(line))
        .collect();

    // Simulate pulse propagation
    let mut pulses: HashMap<String, Pulse> = HashMap::new();
    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;

    for _ in 0..1000 {
        for module in &modules {
            let pulse = module.process_pulse(&mut pulses);

            if pulse == Pulse::Low {
                total_low_pulses += 1;
            } else {
                total_high_pulses += 1;
            }
        }
    }

    // Calculate the result
    let result = total_low_pulses * total_high_pulses;

    println!("Result: {}", result);
}

