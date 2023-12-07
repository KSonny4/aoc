use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct CardTuple {
    cards: Vec<String>,
    cards_numbers: Vec<i32>,
    bid: i32,
}

fn get_bigger_card(card1: &CardTuple, card2: &CardTuple) -> Ordering {
    for (num1, num2) in card1.cards_numbers.iter().zip(&card2.cards_numbers) {
        match num1.cmp(num2) {
            Ordering::Equal => (),
            ordering => return ordering,
        }
    }
    panic!("2 Cards are the same!")
}



fn create_histogram(numbers: &[i32]) -> HashMap<i32, usize> {
    let mut histogram = HashMap::new();

    for &num in numbers {
        *histogram.entry(num).or_insert(0) += 1;
    }

    histogram
}

fn has_4_and_1_card(cards: &HashMap<i32,usize>) -> bool {
    cards.values().any(|&value| value == 4)
        && cards.values().all(|&value| value == 1 || value == 4)
}

fn has_3_and_two_distinct_cards(cards: &HashMap<i32,usize>) -> bool {
    cards.values().any(|&value| value == 3)
        &&     cards.values().all(|&value| value == 3 || value == 1)
}

fn has_full_house(cards: &HashMap<i32,usize>) -> bool {
    cards.values().all(|&value| value == 3 || value == 2)
}
fn has_two_pairs(cards: &HashMap<i32,usize>) -> bool {
    // 2,2,1
    cards.values().filter(|&&value| value == 2 || value == 1).count() == 3
}

fn has_one_pair(cards:& HashMap<i32,usize>) -> bool {
    // 2,1,1,1
    cards.values().filter(|&&value| value == 2 || value == 1).count() == 4
}

fn has_high_card(cards: &HashMap<i32, usize>) -> bool {
    cards.values().all(|&value| value == 1)
}



fn get_type_for_hand(card: &CardTuple) -> i32 {
    match create_histogram(&card.cards_numbers) {
        five_of_a_kind if five_of_a_kind.len() == 1 => 7,
        four_of_a_kind if has_4_and_1_card(&four_of_a_kind) => 6,
        full_house if has_full_house(&full_house) => 5,
        three_of_a_kind if has_3_and_two_distinct_cards(&three_of_a_kind) => 4,
        two_pairs if has_two_pairs(&two_pairs) => 3,
        one_pair if has_one_pair(&one_pair) => 2,
        high_card if has_high_card(&high_card) =>1 ,
        _ => panic!("Card not recognized")
    }
}




impl PartialOrd for CardTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let card1 = get_type_for_hand(&self);
        let card2 = get_type_for_hand(&other);

        match card1.cmp(&card2) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(get_bigger_card(self, other)),
        }
    }
}

fn main() {
    let mut mapping: HashMap<String, i32> = HashMap::new();

    for i in 2..=10 {
        mapping.insert(i.to_string(), i);
    }

    mapping.insert("T".to_string(), 10);
    mapping.insert("J".to_string(), 11);
    mapping.insert("Q".to_string(), 12);
    mapping.insert("K".to_string(), 13);
    mapping.insert("A".to_string(), 14);

    // println!("{:?}", mapping);

    let mut data: Vec<CardTuple> = include_str!("input.txt")
        .lines()
        .map(|section| {
            let parts: Vec<&str> = section.trim().split_whitespace().collect();
            let cards: Vec<String> = parts[0]
                .chars()
                .map(|c| c.to_string())
                .collect();
            let cards_numbers: Vec<i32> = cards
                .iter()
                .map(|card| *mapping.get(card).unwrap())
                .collect();
            let bid: i32 = parts[1].parse().unwrap();

            CardTuple {
                cards,
                cards_numbers,
                bid,
            }
        })
        .collect();

    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let gg: i32 = data.iter().enumerate().map(|(index, x)| (index+1) as i32 * x.bid).sum();

    // println!("{:?}", data);
    println!("{:?}", gg);
}
