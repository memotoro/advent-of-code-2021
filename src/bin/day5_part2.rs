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

        let delta = check_diagonal_lines(&coords_origin, &coords_dest);

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
        } else if delta > 0 {
            let mut start_x: i32;
            let mut start_y: i32;
            let mut increase_x = false;
            let mut increase_y = false;

            start_x = coords_origin[0];
            start_y = coords_origin[1];

            if coords_origin[0] < coords_dest[0] {
                increase_x = true;
            }
            if coords_origin[1] < coords_dest[1] {
                increase_y = true;
            }

            let mut counter = 0;
            while counter <= delta {
                field.grid[start_y as usize][start_x as usize] += 1;
                if increase_x {
                    start_x += 1;
                } else {
                    start_x -= 1;
                }
                if increase_y {
                    start_y += 1;
                } else {
                    start_y -= 1;
                }
                counter += 1;
            }
        }
    }

    for row in &field.grid {
        println!("{:?}", row);
    }

    field.find_overlaps()
}

fn check_diagonal_lines(coords_origin: &Vec<i32>, coords_dest: &Vec<i32>) -> u32 {
    let delta_x = coords_origin[0] - coords_dest[0];
    let delta_y = coords_origin[1] - coords_dest[1];

    let mut delta = 0;

    if delta_x.abs() == delta_y.abs() {
        delta = delta_x.abs();
    }

    delta as u32
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
