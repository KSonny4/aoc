// // use std::collections::{HashMap, HashSet};
// //
// // #[derive(Debug, Clone, PartialEq)]
// // enum Pulse {
// //     High,
// //     Low,
// // }
// //
// // enum Module {
// //     FlipFlop(String),
// //     Conjunction(String, Vec<Module>),
// //     Broadcaster(String, Vec<Module>),
// // }
// //
// // trait ModuleTrait {
// //     fn process_pulse(&self, from_module: dyn ModuleTrait, pulse: Pulse);
// //
// //     fn get_connections(&self) -> Vec<Module>;
// //     fn send (&self, pulse_to_send: Pulse) {
// //         self.get_connections().iter().for_each(|connection| {
// //             connection.process_pulse(pulse_to_send);
// //         });
// //     }
// // }
// //
// // #[derive(Debug, Clone)]
// // struct FlipFlop {
// //     name: String,
// //     on: bool,
// //     connections: Vec<Pulse>,
// //     high_pulse_count: i64,
// //     low_pulse_count: i64,
// // }
// //
// // #[derive(Debug, Clone)]
// // struct Conjunction {
// //     name: String,
// //     on: bool,
// //     last_pulse_from_conjunctions: HashMap<dyn ModuleTrait, Pulse>,
// //     initialized: bool,
// //     connections: Vec<Module>
// // }
// //
// // #[derive(Debug, Clone)]
// // struct Broadcaster {
// //     name: String,
// //     connections: Vec<Module>
// // }
// //
// // impl ModuleTrait for FlipFlop {
// //     fn process_pulse(&mut self, from_module: Box<dyn ModuleTrait>, pulse: Pulse) {
// //
// //         match pulse {
// //             Pulse::High => {
// //                 self.high_pulse_count += 1;
// //             }
// //             Pulse::Low => {
// //                 self.low_pulse_count += 1;
// //                 if self.on {
// //                     self.on = false;
// //                     //Pulse::Low
// //                 } else {
// //                     self.on = true;
// //                     self.send(Pulse::High);
// //                 }
// //             }
// //         }
// //         panic!()
// //     }
// //     fn get_connections(&self) -> Vec<Pulse> {
// //         return self.clone().connections
// //     }
// // }
// //
// // impl ModuleTrait for Conjunction {
// //     fn process_pulse(&mut self, from_module: Box<dyn ModuleTrait>, pulse: Pulse) {
// //         self.last_pulse_from_conjunctions
// //             .entry(from_module)
// //             .and_modify(|existing_value| {
// //                 // Update the existing value
// //                 *existing_value = pulse;
// //             })
// //             .or_insert(Pulse::Low);
// //
// //         // if all values are high, send a high pulse
// //         if self.last_pulse_from_conjunctions.values().all(|&p| p == Pulse::High) {
// //             self.send(Pulse::Low);
// //         } else {
// //             self.send(Pulse::High);
// //         }
// //
// //     }
// //
// //
// //     fn get_connections(&self) -> Vec<Pulse> {
// //         return self.clone().connections
// //     }
// //
// // }
// //
// //
// //
// //
// // impl ModuleTrait for Broadcaster {
// //     fn process_pulse(&mut self, from_module: ModuleTrait, pulse: Pulse) {
// //         self.send(pulse);
// //
// //     }
// //
// //
// //     fn get_connections(&self) -> Vec<Pulse> {
// //         return self.clone().connections
// //     }
// //
// // }
// // fn main() {
// //     // Your module configuration input
// //     let input = "
// //     broadcaster -> a, b, c
// //     %a -> b
// //     %b -> c
// //     %c -> inv
// //     &inv -> a
// //     ";
// //
// //     // Parse the input to create module instances
// //     let modules: Vec<Module> = input
// //         .lines()
// //         .filter(|line| !line.trim().is_empty())
// //         .map(|line| Module::new(line))
// //         .collect();
// //
// //     // Simulate pulse propagation
// //     let mut pulses: HashMap<String, Pulse> = HashMap::new();
// //     let mut total_low_pulses = 0;
// //     let mut total_high_pulses = 0;
// //
// //     }
// //
// //     // Calculate the result
// //     //let result = total_low_pulses * total_high_pulses;
// //
// //     //println!("Result: {}", result);
// // }
// //
//
//
// use std::collections::{HashMap, HashSet};
//
// #[derive(Debug, Clone, PartialEq)]
// enum Pulse {
//     High,
//     Low,
// }
//
// enum Module {
//     FlipFlop(String),
//     Conjunction(String, HashSet<String>),
//     Broadcaster(String, HashSet<String>),
// }
//
// trait ModuleTrait {
//     fn process_pulse(&self, from_module: Box<dyn ModuleTrait>, pulse: Pulse);
//
//     fn get_connections(&self) -> Vec<Module>;
//     fn send(&self, from_module: Box<dyn ModuleTrait>, pulse_to_send: Pulse) {
//         self.get_connections().iter().for_each(|connection| {
//             connection.process_pulse(Box::new(self.clone()), pulse_to_send.clone());
//         });
//     }
// }
//
// #[derive(Debug, Clone)]
// struct FlipFlop {
//     name: String,
//     on: bool,
//     connections: Vec<Module>,
//     high_pulse_count: i64,
//     low_pulse_count: i64,
// }
//
// #[derive(Debug, Clone)]
// struct Conjunction {
//     name: String,
//     on: bool,
//     last_pulse_from_conjunctions: HashMap<Box<dyn ModuleTrait>, Pulse>,
//     initialized: bool,
//     connections: Vec<Module>,
// }
//
// #[derive(Debug, Clone)]
// struct Broadcaster {
//     name: String,
//     connections: Vec<Module>,
// }
//
// impl ModuleTrait for FlipFlop {
//     fn process_pulse(&mut self, from_module: Box<dyn ModuleTrait>, pulse: Pulse) {
//         match pulse {
//             Pulse::High => {
//                 self.high_pulse_count += 1;
//             }
//             Pulse::Low => {
//                 self.low_pulse_count += 1;
//                 if self.on {
//                     self.on = false;
//                     //Pulse::Low
//                 } else {
//                     self.on = true;
//                     self.send(Box::new(self.clone()), Pulse::High);
//                 }
//             }
//         }
//         // Handle the panic!() appropriately
//     }
//
//     fn get_connections(&self) -> Vec<Module> {
//         self.connections.clone()
//     }
// }
//
// impl ModuleTrait for Conjunction {
//     fn process_pulse(&mut self, from_module: Box<dyn ModuleTrait>, pulse: Pulse) {
//         self.last_pulse_from_conjunctions
//             .entry(from_module)
//             .and_modify(|existing_value| {
//                 // Update the existing value
//                 *existing_value = pulse;
//             })
//             .or_insert(Pulse::Low);
//
//         // if all values are high, send a high pulse
//         if self.last_pulse_from_conjunctions.values().all(|&p| p == Pulse::High) {
//             self.send(Box::new(self.clone()), Pulse::High);
//         } else {
//             self.send(Box::new(self.clone()), Pulse::Low);
//         }
//     }
//
//     fn get_connections(&self) -> Vec<Module> {
//         self.connections.clone()
//     }
// }
//
// impl ModuleTrait for Broadcaster {
//     fn process_pulse(&self, from_module: Box<dyn ModuleTrait>, pulse: Pulse) {
//         self.send(Box::new(self.clone()), pulse);
//     }
//
//     fn get_connections(&self) -> Vec<Module> {
//         self.connections.clone()
//     }
// }
//
// fn main() {
//     // Your module configuration input
//     let input = "
//     broadcaster -> a, b, c
//     %a -> b
//     %b -> c
//     %c -> inv
//     &inv -> a
//     ";
//
//     // Parse the input to create module instances
//     let modules: Vec<Module> = input
//         .lines()
//         .filter(|line| !line.trim().is_empty())
//         .map(|line| Module::new(line))
//         .collect();
//
//     // Simulate pulse propagation
//     let mut pulses: HashMap<String, Pulse> = HashMap::new();
//     let mut total_low_pulses = 0;
//     let mut total_high_pulses = 0;
//
//     // Implement pulse propagation simulation
//
//     // Calculate the result
//     // let result = total_low_pulses * total_high_pulses;
//
//     // println!("Result: {}", result);
// }


use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(String, bool), // The second parameter represents the on/off state
    Conjunction(String, HashSet<String>),
    Broadcaster(String, HashSet<String>),
}

impl Module {
    fn new(name: &str, connections: HashSet<String>) -> Self {
        match name.chars().next() {
            Some('%') => Module::FlipFlop(name.to_string(), false),
            Some('&') => Module::Conjunction(name.to_string(), connections),
            _ => Module::Broadcaster(name.to_string(), connections),
        }
    }
}

trait ModuleTrait {
    fn process_pulse(&mut self, pulse: Pulse);
    fn get_connections(&self) -> Vec<String>;
    fn send(&self, pulse_to_send: Pulse) {
        self.get_connections().iter().for_each(|connection| {
            // Simulating sending pulse to connected modules
            // You may need to implement the actual logic based on the module type
            // For now, just print the connection and pulse for demonstration purposes
            println!("{} -> {:?}", connection, pulse_to_send);
        });
    }
}

impl ModuleTrait for Module {
    fn process_pulse(&mut self, pulse: Pulse) {
        match self {
            Module::FlipFlop(_, on) => {
                match pulse {
                    Pulse::High => {
                        // Ignore high pulse for FlipFlop
                    }
                    Pulse::Low => {
                        *on = !*on;
                        if *on {
                            self.send(Pulse::High);
                        }
                    }
                }
            }
            Module::Conjunction(_, connections) => {
                connections.iter().for_each(|connection| {
                    // Simulate sending the pulse to connected modules
                    // You may need to implement the actual logic based on the module type
                    // For now, just print the connection and pulse for demonstration purposes
                    println!("{} -> {:?}", connection, pulse);
                });

                // Implement the Conjunction logic based on the most recent pulses received
                // Update the state and send pulses accordingly
                // For now, just print the pulse for demonstration purposes
                println!("{:?} Conjunction processing pulse", pulse);
            }
            Module::Broadcaster(_, connections) => {
                // Implement Broadcaster pulse processing logic here
                // Send the pulse to all connected modules
                connections.iter().for_each(|connection| {
                    // Simulate sending the pulse to connected modules
                    // You may need to implement the actual logic based on the module type
                    // For now, just print the connection and pulse for demonstration purposes
                    println!("{} -> {:?}", connection, pulse);
                });
            }
        }
    }

    fn get_connections(&self) -> Vec<String> {
        match self {
            Module::FlipFlop(_, _) => Vec::new(), // FlipFlop has no connections
            Module::Conjunction(_, connections) | Module::Broadcaster(_, connections) => {
                connections.iter().cloned().collect()
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
        .map(|line| {
            let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
            let name = parts[0];
            let connections: HashSet<String> = parts[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            Module::new(name, connections)
        })
        .collect();

    // Simulate pulse propagation
    for _ in 0..1000 {
        // Simulate pushing the button 1000 times
        for module in &modules {
            // Simulate sending a low pulse to the broadcaster module
            module.process_pulse(Pulse::Low);
        }
        // Wait for all pulses to be fully handled before pushing the button again
    }

    // Calculate the result
    // TODO: Implement result calculation based on the simulated pulses
    // For now, just print a placeholder message
    println!("Placeholder: Result calculation not implemented");
}
