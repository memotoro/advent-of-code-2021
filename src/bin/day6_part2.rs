use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY6_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let data = contents.lines().collect::<Vec<&str>>();

    println!("number of lanternfish {}", number_of_lanternfish(data));
}

fn number_of_lanternfish(data: Vec<&str>) -> u32 {
    let days = 80;
    let mut control = 0;

    let fish_input = data[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut freq: HashMap<i32, u32> = HashMap::new();

    for f in fish_input {
        freq.insert(f, *freq.get(&f).unwrap());
    }

    println!("{:?}", freq);

    while control < days {
        for (key, _) in freq.iter_mut() {
            let value = match freq.get(key) {
                Some(v) => v,
                None => &0,
            };

            if value - 1 < 0 {
                let value_6 = match freq.get(&6) {
                    Some(v) => v + 1,
                    None => 0,
                };
                let value_8 = match freq.get(&8) {
                    Some(v) => v + 1,
                    None => 0,
                };
                let new_key = key - 1;
                let new_key_value = match freq.get(&new_key) {
                    Some(v) => v,
                    None => &0,
                };
                freq.insert(6, value_6);
                freq.insert(8, value_8);
                freq.insert(*key, value - 1);
                freq.insert(new_key, *new_key_value);
            } else {
                freq.insert(*key, *value);
            }
        }
        control += 1;
        println!("{:?}", freq);
    }

    0
}
