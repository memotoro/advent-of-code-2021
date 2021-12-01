use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY1_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let measurements = contents
        .lines()
        .map(|d| d.parse::<u32>().expect("invalid input"))
        .collect::<Vec<u32>>();

    println!(
        "measurements {}",
        calculate_increase_measurements(&measurements)
    );
}

fn calculate_increase_measurements(measurements: &Vec<u32>) -> u32 {
    let mut counter: u32 = 0;
    let mut index: usize = 2;
    let mut previous: u32 = 0;

    while index < measurements.len() {
        let value = measurements[index] + measurements[index - 1] + measurements[index - 2];

        if index > 2 && value > previous {
            counter += 1;
        }

        previous = value;
        index += 1;
    }

    counter
}
