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
    let oxygen_value = calculate_reading(data.clone(), 1, 0);

    let co2_value = calculate_reading(data, 0, 1);

    oxygen_value * co2_value
}

fn calculate_reading(data: Vec<&str>, select: u32, discard: u32) -> u32 {
    let mut bit = 0;
    let mut readings = data.clone();

    while readings.len() > 1 && bit < readings[0].len() {
        let zeros = zeros_counter(readings.clone(), bit);
        let ones = (readings.len() as i32) - zeros;

        let mut filter = select;
        if zeros > ones {
            filter = discard;
        }

        let mut data_filter: Vec<&str> = Vec::new();

        for d in readings.clone() {
            let raw_reading = d.chars().collect::<Vec<char>>();
            if raw_reading[bit].to_digit(10).unwrap() == filter {
                data_filter.push(d);
            }
        }
        readings = data_filter;

        bit += 1;
    }

    u32::from_str_radix(readings[0], 2).unwrap()
}

fn zeros_counter(data: Vec<&str>, index: usize) -> i32 {
    let mut counter = 0;

    for d in data {
        let raw_reading = d.chars().collect::<Vec<char>>();
        if raw_reading[index].to_digit(10).unwrap() == 0 {
            counter += 1;
        };
    }

    counter
}
