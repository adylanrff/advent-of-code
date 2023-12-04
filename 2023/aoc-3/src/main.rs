use std::fs;

#[derive(Debug)]
struct Schematic {
    raw: Vec<Vec<char>>,
    numbers: Vec<i32>,
    gear_numbers: Vec<i32>,
    visited: Vec<Vec<bool>>,
    local_visited: Vec<Vec<bool>>,

    height: usize,
    width: usize,
}

impl Schematic {
    fn from_file(file_path: &str) -> Self {
        let file_lines: Vec<Vec<char>> = fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let height = file_lines.len();
        let width = file_lines[0].len();

        let visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let local_visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

        Schematic {
            raw: file_lines,
            visited: visited,
            local_visited: local_visited,
            numbers: Vec::new(),
            gear_numbers: Vec::new(),

            height: height,
            width: width,
        }
    }

    fn parse(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let ch: char = self.raw[i][j];
                // if is symbol
                if !ch.is_numeric() && ch != '.' {
                    self.explore_symbol(i32::try_from(i).unwrap(), i32::try_from(j).unwrap());
                }
            }
        }
    }

    fn explore_symbol(&mut self, i: i32, j: i32) {
        let directions: [(i32, i32); 8] = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        let mut numbers: Vec<i32> = Vec::new();
        let mut gear_numbers: Vec<i32> = Vec::new();
        let i_idx: usize = i.try_into().unwrap();
        let j_idx: usize = j.try_into().unwrap();

        if self.raw[i_idx][j_idx] == '*' {
            println!("{}:{}", i_idx, j_idx)
        }

        self.local_visited = vec![vec![false; self.width]; self.height];

        for dir in directions {
            let y = dir.0 + i;
            let x = dir.1 + j;

            if (x >= self.width.try_into().unwrap() || x < 0)
                || (y >= self.height.try_into().unwrap() || y < 0)
            {
                continue;
            }

            if let Some(number) = self.get_number(y, x, true) {
                numbers.push(number)
            }

            if self.raw[i_idx][j_idx] == '*' {
                if let Some(number) = self.get_number(y, x, false) {
                    gear_numbers.push(number)
                }
            }
        }

        if gear_numbers.len() == 2 {
            self.gear_numbers.push(gear_numbers[0] * gear_numbers[1]);
        }

        self.numbers.append(&mut numbers);
    }

    fn get_number(&mut self, i: i32, j: i32, check_visited: bool) -> Option<i32> {
        let mut num_str = String::new();
        let i_idx: usize = i.try_into().unwrap();
        let j_idx: usize = j.try_into().unwrap();

        if !self.raw[i_idx][j_idx].is_numeric() {
            return None;
        }

        // explore left side
        let mut cur_j_idx = j_idx;
        while self.raw[i_idx][cur_j_idx].is_numeric() {
            if check_visited && self.visited[i_idx][cur_j_idx] {
                break;
            } else {
                if self.local_visited[i_idx][cur_j_idx] {
                    break;
                }
            }

            if check_visited {
                self.visited[i_idx][cur_j_idx] = true;
            } else {
                self.local_visited[i_idx][cur_j_idx] = true;
            }

            num_str = format!("{}{}", self.raw[i_idx][cur_j_idx], num_str);

            if cur_j_idx == 0 {
                break;
            }
            cur_j_idx -= 1;
        }

        // explore right side
        cur_j_idx = j_idx + 1;
        while cur_j_idx < self.width && self.raw[i_idx][cur_j_idx].is_numeric() {
            if check_visited && self.visited[i_idx][cur_j_idx] {
                break;
            } else {
                if self.local_visited[i_idx][cur_j_idx] {
                    break;
                }
            }

            if check_visited {
                self.visited[i_idx][cur_j_idx] = true;
            } else {
                self.local_visited[i_idx][cur_j_idx] = true;
            }

            num_str = format!("{}{}", num_str, self.raw[i_idx][cur_j_idx]);
            cur_j_idx += 1;
        }

        if num_str != "" && !check_visited {
            println!("\t({},{}):{}", i, j, num_str);
        }

        if let Ok(result) = num_str.parse::<i32>() {
            return Some(result);
        }

        None
    }
}

fn main() {
    let file_path = "./data/input.txt";
    let mut schematic = Schematic::from_file(file_path);
    schematic.parse();
    let mut result: i64 = 0;
    for num in schematic.numbers {
        result += i64::from(num);
    }

    let mut gear_number_result: i64 = 0;
    for num in schematic.gear_numbers {
        gear_number_result += i64::from(num);
    }

    println!("1. result: {}", result);
    println!("2. result: {}", gear_number_result);
}
