


    #[derive(Debug, Clone)]
    struct Game {
        red: i32,
        green: i32,
        blue: i32,
    }


    fn group(line: &str, possibility_to_check: &Game) -> bool {

        let game_string: Vec<&str> = line.split(": ").nth(1).unwrap().split(";").collect();
        //println!("{:?}", games_string);
        let game_parsed: Vec<Vec<&str>> = game_string
            .iter()
            .map(|set| set.split(',')
                .map(|set_element| set_element
                    .trim())
                .collect())
            .collect();
        //println!("{:?}", gg);

        let mut game_not_possible = false;

        for set in game_parsed.iter() {
            for combination in set.iter(){
                if game_not_possible {
                    return game_not_possible
                }
                match combination.split(" ").collect::<Vec<&str>>().as_slice() {
                    [num, color] => {
                        let num = num.parse::<i32>().unwrap_or(0);
                        match *color {
                            "red" => game_not_possible = num > possibility_to_check
                                .red,
                            "blue" => game_not_possible = num > possibility_to_check
                                .blue,
                            "green" => game_not_possible = num > possibility_to_check
                                .green,
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),

                }
            }

        }
        false


    }

    fn main() {

        let possibility_to_check = Game {red: 12, green: 13, blue: 14};

        let res: usize = include_str!("input.txt")
            .lines()
            .map(|line| group(line, &possibility_to_check))
            .enumerate()
            .map(|(line_no, game_not_possible)| {
                if game_not_possible {
                    println!("{} not possible", line_no + 1);
                    0
                } else {
                    println!("{} possible", line_no + 1);
                    line_no + 1
                }
            })
            .sum();

        print!("{}", res);
    }
