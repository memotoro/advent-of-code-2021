use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY5_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let data = contents.lines().collect::<Vec<&str>>();

    println!("dangerous areas {}", calculate_dangerous_areas(data));
}

fn calculate_dangerous_areas(data: Vec<&str>) -> u32 {
    let mut field = Field::new();

    for d in data {
        let segments = d.split(" -> ").collect::<Vec<&str>>();
        let origin = segments[0];
        let destination = segments[1];

        let coords_origin = origin
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let coords_dest = destination
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if coords_origin[0] == coords_dest[0] || coords_origin[1] == coords_dest[1] {
            let mut start: i32;
            let end: i32;
            let mut x_axis_movement = false;
            let mut check_index = 1;

            if coords_origin[1] == coords_dest[1] {
                x_axis_movement = true;
                check_index = 0;
            }

            if coords_origin[check_index] < coords_dest[check_index] {
                start = coords_origin[check_index];
                end = coords_dest[check_index];
            } else {
                start = coords_dest[check_index];
                end = coords_origin[check_index];
            }

            if x_axis_movement {
                while start <= end {
                    field.grid[coords_origin[1] as usize][start as usize] += 1;
                    start += 1;
                }
            } else {
                while start <= end {
                    field.grid[start as usize][coords_origin[0] as usize] += 1;
                    start += 1;
                }
            }
        }
    }

    for row in &field.grid {
        println!("{:?}", row);
    }

    field.find_overlaps()
}

struct Field {
    grid: Vec<Vec<u32>>,
}

impl Field {
    fn new() -> Self {
        let max_x = 1000;
        let max_y = 1000;
        let mut y = 0;
        let mut grid: Vec<Vec<u32>> = Vec::new();

        while y < max_y {
            let mut x = 0;
            let mut row: Vec<u32> = Vec::new();

            while x < max_x {
                row.push(0);
                x += 1;
            }

            grid.push(row);
            y += 1;
        }

        Self { grid }
    }

    fn find_overlaps(&self) -> u32 {
        let mut counter = 0;
        for row in &self.grid {
            for cell in row {
                if *cell > 1 {
                    counter += 1;
                }
            }
        }
        counter
    }
}
