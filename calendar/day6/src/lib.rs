use std::{collections::VecDeque};

pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("6");

    let part_1_answer = get_part_1(&input);
    let part_2_answer = get_part_2(&input);

    (part_1_answer, part_2_answer)
}

fn get_part_1(input: &String) -> String {
    let marker_size = 4;
    calculate_marker_start(input, marker_size)
}

fn get_part_2(input: &String) -> String {
    let marker_size = 14;
    calculate_marker_start(input, marker_size)
}

fn calculate_marker_start(input: &String, marker_size: i32) -> String {
    let mut com_system = ComSystem::new(marker_size);

    let mut marker_start = 0;
    for letter in input.chars() {
        com_system.try_feed_letter(letter);
        let res_option = com_system.get_marker_start_if_valid();

        if res_option.is_some() {
            marker_start = *res_option.unwrap();
            break;
        }
    }

    marker_start.to_string()
}

struct ComSystem {
    max_size: i32,
    marker: VecDeque<char>,
    symbols_checked: i32
}

impl ComSystem {
    fn new(max_size: i32) -> Self {
        ComSystem { max_size, marker: VecDeque::with_capacity(max_size as usize), symbols_checked: 0 }
    }

    fn try_feed_letter(&mut self, letter: char) -> bool {
        if self.marker.len() >= self.max_size as usize {
            return false;
        }

        self.marker.push_back(letter);
        self.symbols_checked += 1;
        true
    }

    fn get_marker_start_if_valid(&mut self) -> Option<&i32> {
        if &self.marker.len() != &(self.max_size as usize) {
            return None;
        }

        for letter in &self.marker {
            let count = &self.marker.iter().filter(|x| *x == letter).count();

            if count != &1 {
                self.marker.pop_front();
                return None;
            }
        }

        return Some(&self.symbols_checked);
    }
}