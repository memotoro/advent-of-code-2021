use dotenv::dotenv;
use std::env;
use std::fs::read_to_string;

fn main() {
    dotenv().ok();

    let input_file_path = env::var("DAY4_PART1_INPUT_FILE").expect("error getting env variable");

    let contents = read_to_string(input_file_path).expect("Something went wrong reading the file");

    let data = contents.lines().collect::<Vec<&str>>();

    println!("bingo score {}", calculate_bingo_score(data));
}

fn calculate_bingo_score(data: Vec<&str>) -> u32 {
    let mut draw_numbers: Vec<u32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut board: Board = Board::new();
    let mut idx = 0;
    let mut row_index = 0;
    let mut score = 0;

    for d in data {
        if idx == 0 {
            draw_numbers = d
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            idx += 1;
            continue;
        }

        if d.len() == 0 {
            boards.push(board);
            board = Board::new();
            row_index = 0;
            continue;
        }

        board.add_numbers(d, row_index);
        row_index += 1;
    }

    boards.remove(0);
    boards.push(board);

    for (index, board) in boards.iter().enumerate() {
        println!("{}", index);
        board.print();
        println!();
    }

    println!("{:?}", draw_numbers);
    println!();

    'outer: for dn in draw_numbers {
        for board in boards.iter_mut() {
            if !board.completed && board.verify_number(dn) {
                board.verify_completion(dn);
                if board.completed {
                    score = board.calculate_score();
                    break 'outer;
                }
            }
        }
    }

    for (index, board) in boards.iter().enumerate() {
        println!("{}", index);
        board.print();
        println!();
    }

    score
}

#[derive(Clone)]
struct Board {
    numbers: [[Cell; 5]; 5],
    completed: bool,
    completed_number: u32,
}

#[derive(Clone)]
struct Cell {
    value: u32,
    checked: bool,
}

impl Board {
    fn new() -> Self {
        Self {
            numbers: Board::default_matrix(),
            completed: false,
            completed_number: 0,
        }
    }

    fn add_numbers(&mut self, data: &str, row_index: u32) {
        let mut numbers: [Cell; 5] = Board::default_row();
        let mut index = 0;

        for d in data.split(" ").collect::<Vec<&str>>() {
            match d.parse::<u32>() {
                Ok(v) => {
                    numbers[index] = Cell {
                        value: v,
                        checked: false,
                    };
                    index += 1;
                }
                Err(_) => continue,
            }
        }

        self.numbers[row_index as usize] = numbers;
    }

    fn verify_number(&mut self, number: u32) -> bool {
        let mut found: bool = false;

        'outer: for row in self.numbers.iter_mut() {
            for cell in row {
                if cell.value == number {
                    cell.checked = true;
                    found = true;
                    break 'outer;
                }
            }
        }

        found
    }

    fn verify_completion(&mut self, number: u32) {
        let mut counter_checked: u32 = 0;

        for row in self.numbers.iter_mut() {
            for cell in row.clone() {
                if cell.checked {
                    counter_checked += 1;
                } else {
                    break;
                }
            }

            if counter_checked == (row.len() as u32) {
                self.completed = true;
                self.completed_number = number;
            } else {
                counter_checked = 0;
            }
        }

        if !self.completed {
            counter_checked = 0;
            let mut col = 0;
            
            while col < self.numbers[0].len() {
                let mut row = 0;
                while row < self.numbers.len() {
                    let cell = &self.numbers[row][col];
                    if cell.checked {
                        counter_checked += 1;
                    } else {
                        break;
                    }
                    row += 1;
                }

                if counter_checked == (self.numbers[col].len() as u32) {
                    self.completed = true;
                    self.completed_number = number;
                    break;
                } else {
                    counter_checked = 0;
                }
                col += 1;
            }
        }
    }

    fn calculate_score(&self) -> u32 {
        let mut sum: u32 = 0;
        for row in &self.numbers {
            for data in row {
                if !data.checked {
                    sum += data.value;
                }
            }
        }

        println!("{} {}", sum, self.completed_number);
        println!();

        sum * self.completed_number
    }

    fn print(&self) {
        for row in &self.numbers {
            let mut data: String = String::from("");
            for cell in row {
                data.push_str(cell.value.to_string().as_str());
                data.push_str(":");
                data.push_str(cell.checked.to_string().as_str());
                data.push_str(" ");
            }
            println!("{}", data);
        }
    }

    fn default_matrix() -> [[Cell; 5]; 5] {
        [
            Board::default_row(),
            Board::default_row(),
            Board::default_row(),
            Board::default_row(),
            Board::default_row(),
        ]
    }

    fn default_cell() -> Cell {
        Cell {
            value: 0,
            checked: false,
        }
    }

    fn default_row() -> [Cell; 5] {
        let cell = Board::default_cell();
        [
            cell.clone(),
            cell.clone(),
            cell.clone(),
            cell.clone(),
            cell.clone(),
        ]
    }
}
