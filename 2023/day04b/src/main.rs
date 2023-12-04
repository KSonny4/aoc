
use std::collections::{HashMap, HashSet, VecDeque};



fn main() {
    let data: Vec<Vec<HashSet<String>>> = include_str!("input.txt")
        .lines()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap_or_default()
                .split('|')
                .map(|part| part.trim().split_whitespace().map(String::from).collect())
                .collect()
        })
        .collect();

    println!("{:?}", data);

    let wins: Vec<i32> = data.iter().map(|sets| match sets.len() {
        2 => sets[0].intersection(&sets[1]).count() as i32,
        _ => 0,
    }).collect();
    println!("Wins: {:?}", wins);


    let mut stack: VecDeque<usize> = VecDeque::new();

    let mut counter = 0; // for 1st instance of card

    (0..wins.len()).into_iter().for_each(|x| {
        stack.push_back(x);
    });



    while !stack.is_empty() {
        counter += 1;
        let index: usize = stack.pop_front().unwrap_or(9999999);
        //println!("Card: {}, index: {}", index+1, index);

        let score = wins[index];
        let next_cards: Vec<usize> = (index + 1 + 1..score as usize + index + 2).collect();

        next_cards.iter().for_each(|x| {
            stack.push_back(*x-1);
        });

       // println!("Next cards from {} -> {:?}. Stack: {:?}. Counter: {:?}", index , next_cards, stack, counter);

    }

    println!("{}", counter);
}
