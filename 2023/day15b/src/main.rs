use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn hash(line: Vec<char>) -> u32 {
    let mut current_val: u32 = 0;
    line.iter().for_each(|&c| {
        current_val += c as u32;
        current_val = (current_val * 17) % 256;
    });
    current_val
}

fn main() {
    // load rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7 into a vec of vec of chars
    let data: Vec<String> = include_str!("input.txt")
        .split(",")
        .map(|line| line.to_string())
        .collect();

    let mut hashes: HashMap<u32, Vec<Lens>> = HashMap::new();

    data.iter().for_each(|line| {
        match line {
            x if line.contains("=") => {
                let val: Vec<&str> = x.split("=").collect();
                let label = val[0].to_string();
                let focal_length = val[1].parse().unwrap();
                let lens = Lens {
                    label: label.clone(),
                    focal_length,
                };

                let label_hash = hash(label.chars().collect());

                if let Some(existing_lens) = hashes.get_mut(&label_hash) {
                    if let Some(existing_lens_mut) = existing_lens.iter_mut().find(|l| l.label == label) {
                        existing_lens_mut.focal_length = focal_length;
                    } else {
                        existing_lens.push(lens);
                    }
                } else {
                    hashes.insert(label_hash, vec![lens]);
                }
            }
            x if line.contains("-") => {
                let label = x.replace("-", "");
                let label_hash = hash(label.chars().collect());
                if let Some(existing_lens) = hashes.get_mut(&label_hash) {
                    existing_lens.retain(|lens| lens.label != label);
                }
            }
            _ => panic!("Invalid input"),
        }
    });

    println!("{:#?}", hashes);


    let mut sorted_keys: Vec<u32> = hashes.keys().cloned().collect();
    sorted_keys.sort();

    let mut res_vec = Vec::new();
    // Iterate over sorted keys and access corresponding values
    for key in sorted_keys.iter() {
        if let Some(value) = hashes.get(&key) {
            for (ind2, v) in value.iter().enumerate() {
                res_vec.push(((key + 1) as u32) * ((ind2 + 1) as u32) * v.focal_length);

            }
        }
    }


    println!("{:#?}", res_vec.iter().sum::<u32>()   );



}
