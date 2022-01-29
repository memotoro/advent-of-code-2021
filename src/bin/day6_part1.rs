use dotenv::dotenv;
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

    let mut fish: Vec<i32> = Vec::new();

    for f in fish_input {
        fish.push(f);
    }

    println!("{:?}", fish);

    while control < days {
        let mut new = 0;
        for f in fish.iter_mut() {
            *f -= 1;
            if *f < 0 {
                *f = 6;
                new += 1;
            }
        }
        for _i in 0..new {
            fish.push(8);
        }
        control += 1;
        println!("{:?}", fish);
    }

    fish.len() as u32
}
