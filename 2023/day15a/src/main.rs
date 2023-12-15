fn main() {
    // load rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7 into a vec of vec of chars
    let data: Vec<Vec<char>> = include_str!("input.txt")
        .split(",")
        .map(|line| line.chars().collect())
        .collect();

    let res: u32 = data
        .iter()
        .map(|line| {
            let mut current_val: u32 = 0;
            line.iter().for_each(|&c| {
                current_val += c as u32;
                current_val = (current_val * 17) % 256;
            });
            current_val
        })
        .sum();

    println!("{:?}", res);
}
