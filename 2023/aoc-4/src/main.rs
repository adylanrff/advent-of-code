use std::{collections::HashSet, fs, i32};

struct Game {
    cards: Vec<Card>,
    card_counts: Vec<u32>,
}

impl Game {
    fn from_file(file_path: &str) -> Self {
        let mut card_vec: Vec<Card> = Vec::new();

        let file_lines: Vec<_> = fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for line in file_lines {
            let card = Card::from_line(&line);
            card_vec.push(card);
        }
        let card_length = card_vec.len();

        Game {
            cards: card_vec,
            card_counts: vec![1; card_length],
        }
    }

    fn calculate_1(&self) -> i32 {
        let mut result = 0;
        for c in &self.cards {
            result += c.calculate_1();
        }
        result
    }

    fn calculate_2(&mut self) -> u32 {
        for c in &self.cards {
            let card_count: usize = c.get_matching_numbers().try_into().unwrap();
            let lower_bound = c.id + 1;
            let upper_bound = lower_bound + card_count;

            for i in lower_bound..upper_bound {
                self.card_counts[i] += self.card_counts[c.id];
            }
        }

        if let Some(result) = self.card_counts.clone().into_iter().reduce(|a, b| a + b) {
            return result;
        }
        return 0;
    }
}

struct Card {
    id: usize,
    winning_numbers: HashSet<i32>,
    numbers_owned: Vec<i32>,
}

impl Card {
    fn from_line(input: &str) -> Self {
        let splitted_input: Vec<_> = input.split(':').collect();

        // card id
        let card_identifier: Vec<_> = splitted_input[0].split_whitespace().collect();
        let card_id = card_identifier[1].parse::<usize>().unwrap() - 1;

        let game = splitted_input[1];
        let splitted_game: Vec<_> = game.split('|').collect();

        // winning numbers
        let winning_numbers: HashSet<i32> = splitted_game[0]
            .split_whitespace()
            .map(|ch| ch.parse::<i32>().unwrap())
            .collect();
        // numbers owned
        let numbers_owned: Vec<i32> = splitted_game[1]
            .split_whitespace()
            .map(|ch| ch.parse::<i32>().unwrap())
            .collect();

        Card {
            id: card_id,
            winning_numbers: winning_numbers,
            numbers_owned: numbers_owned,
        }
    }

    fn calculate_1(&self) -> i32 {
        let count = self.get_matching_numbers();
        if count == 0 {
            return 0;
        }
        return 2_i32.pow(count - 1);
    }

    fn get_matching_numbers(&self) -> u32 {
        let mut count: u32 = 0;
        for num in &self.numbers_owned {
            if self.winning_numbers.contains(num) {
                count += 1
            }
        }
        count
    }
}

fn main() {
    let file_path = "./data/input.txt";
    let mut game = Game::from_file(file_path);
    println!("result 1 = {}", game.calculate_1());
    println!("result 2 = {}", game.calculate_2());
    for (i, cc) in game.card_counts.iter().enumerate() {
        println!("{}: {}", i + 1, cc);
    }
}
