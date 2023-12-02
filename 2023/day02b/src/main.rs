


    #[derive(Debug, Clone)]
    struct Game {
        red: i32,
        green: i32,
        blue: i32,
    }


    fn group(line: &str) -> i32 {

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

        let mut maxs = Game { red: 0, blue: 0, green: 0};
        for set in game_parsed.iter() {
            for combination in set.iter(){
                match combination.split(" ").collect::<Vec<&str>>().as_slice() {
                    [num, color] => {
                        let num = num.parse::<i32>().unwrap_or(0);
                        match *color {
                            "red" => if num > maxs.red {
                                maxs.red = num
                            }
                            "blue" =>  if num > maxs.blue {
                                maxs.blue = num
                            }
                            "green" =>  if num > maxs.green {
                                maxs.green = num
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),

                }
            }

        }
        maxs.green * maxs.red * maxs.blue


    }

    fn main() {

        let possibility_to_check = Game {red: 12, green: 13, blue: 14};

        let res: i32 = include_str!("input.txt")
            .lines()
            .map(|line| group(line))
            .sum();

        print!("{}", res);
    }
