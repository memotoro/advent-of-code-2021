use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY1_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let measurements = contents
        .lines()
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!(
        "measurements {}",
        calculate_increase_measurements(&measurements)
    );
}

fn calculate_increase_measurements(measurements: &Vec<u32>) -> u32 {
    let mut counter: u32 = 0;
    let mut index: usize = 1;

    while index < measurements.len() {
        if measurements[index] > measurements[index - 1] {
            counter += 1;
        }

        index += 1
    }

    counter
}
