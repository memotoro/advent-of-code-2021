use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY2_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let data = contents.lines().collect::<Vec<&str>>();

    println!("planned course {}", calculate_planned_course(data));
}

fn calculate_planned_course(data: Vec<&str>) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for d in data {
        let raw_command = d.split(" ").collect::<Vec<&str>>();
        let movement = raw_command[0];
        let value = raw_command[1].parse::<i32>().unwrap();
        match movement {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        }
    }

    horizontal * depth
}
