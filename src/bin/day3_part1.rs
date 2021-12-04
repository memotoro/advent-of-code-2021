use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY3_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let data = contents.lines().collect::<Vec<&str>>();

    println!("power consumption {}", calculate_power_consumption(data));
}

fn calculate_power_consumption(data: Vec<&str>) -> u32 {
    let length = data[0].len();
    let mut bit = 0;
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    while bit < length {
        let zeros = zeros_counter(&data, bit);

        let mut gamma_digit = "0";
        let mut epsilon_digit = "1";

        if zeros < (data.len() as i32) - zeros {
            gamma_digit = "1";
            epsilon_digit = "0";
        }

        gamma.push_str(gamma_digit);
        epsilon.push_str(epsilon_digit);

        bit += 1;
    }

    let gamma_value = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_value = u32::from_str_radix(epsilon.as_str(), 2).unwrap();

    gamma_value * epsilon_value
}

fn zeros_counter(data: &Vec<&str>, index: usize) -> i32 {
    let mut zeros_counter = 0;

    for d in data {
        let raw_reading = d.chars().collect::<Vec<char>>();
        if raw_reading[index].to_digit(10).unwrap() == 0 {
            zeros_counter += 1;
        };
    }

    zeros_counter
}
