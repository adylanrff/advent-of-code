use std::cmp;
use std::fs;

struct CubeSet {
    r: i32,
    g: i32,
    b: i32,
}

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet { r: 0, g: 0, b: 0 }
    }
}

struct Game {
    id: i32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn create_game(line: &String) -> Game {
        let game_data: Vec<String> = line.split(":").map(String::from).collect();

        // get game id
        let game_header: Vec<String> = game_data[0].split(' ').map(String::from).collect();
        let game_id = game_header[1].parse().unwrap();

        // get cube sets
        let mut cube_sets = Vec::<CubeSet>::new();
        let game_instances: Vec<String> = game_data[1].split(";").map(String::from).collect();
        for game_instance in &game_instances {
            let mut cube_set = CubeSet::new();
            let cube_set_input: Vec<String> = game_instance.split(",").map(String::from).collect();

            for c in &cube_set_input {
                let splitted_cubeset: Vec<String> = c.trim().split(' ').map(String::from).collect();
                let count: i32 = splitted_cubeset[0].parse().unwrap();
                let cube_type_str = &splitted_cubeset[1];

                match cube_type_str.as_str() {
                    "red" => cube_set.r += count,
                    "green" => cube_set.g += count,
                    "blue" => cube_set.b += count,
                    _ => panic!("unexpected"),
                }
            }

            cube_sets.push(cube_set);
        }

        Game {
            id: game_id,
            cube_sets: cube_sets,
        }
    }

    fn verify(&self) -> bool {
        for cube_set in &self.cube_sets {
            if cube_set.r > 12 || cube_set.g > 13 || cube_set.b > 14 {
                return false;
            }
        }

        return true;
    }

    fn get_power(&self) -> i32 {
        let mut max_r: i32 = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for cube_set in &self.cube_sets {
            max_r = cmp::max(max_r, cube_set.r);
            max_g = cmp::max(max_g, cube_set.g);
            max_b = cmp::max(max_b, cube_set.b);
        }

        return max_r * max_g * max_b;
    }
}

fn main() {
    let file_path = "./data/input.txt";
    let file_lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result_1: i32 = 0;
    let mut result_2: i32 = 0;
    for line in file_lines {
        let game = Game::create_game(&line);
        if game.verify() {
            result_1 += game.id
        }
        result_2 += game.get_power();
    }
    println!("1: result: {}", result_1);
    println!("2: result: {}", result_2);
}
