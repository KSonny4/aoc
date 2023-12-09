fn main() {


    let input: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|line| line.trim().split(" ").map(|s| s.parse::<i32>().unwrap_or(0)).collect()).collect();

    println!("{:?}", input);


    let mut games: Vec<Vec<Vec<i32>>> = Vec::new();
    input.iter().for_each(|line| {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        let mut row: Vec<i32> = line.clone();
        rows.push(row.clone());

        while !row.iter().all(|x| x == &0) {
            let mut new_row: Vec<i32> = Vec::new();
            for index in 1..row.len() {
                new_row.push(row[index] - row[index-1]);
            }
            row = new_row.clone();
            rows.push(row.clone());
        }
        games.push(rows);
    });

    //println!("{:?}", games);

    games.iter_mut().for_each(|game|
      {
          let mut last_inserted_val = 0;
          for (index,row) in game.iter_mut().rev().enumerate() {
              if index == 0 {
                  row.push(0);
                  last_inserted_val = 0; // not really needed
              }

              let value_to_add = row.last().unwrap() + last_inserted_val;
              row.push(value_to_add);
              last_inserted_val = value_to_add;

          }
      }

    );

    //println!("{:#?}", games);

    let res: i32 = games.iter().map(|x| x.first().unwrap().last().unwrap()).sum();
    println!("{:?}", res);

    // game.iter_mut().for_each(|g| {
    //     g.iter_mut().rev() {
    //         row.push(0);
    //     });
    // });


}
