use rucksack::{Rucksack, RucksackGroup};

mod rucksack;
mod input_converter;

static LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("3");

    let rucksack_list = input_converter::convert_input_to_rucksakcs(input);

    let first_part_result = part1_task_result(&rucksack_list);
    let second_part_result = part2_task_result(rucksack_list);

    (first_part_result.to_string(), second_part_result.to_string())
}

fn part2_task_result(rucksack_list: Vec<Rucksack>) -> i32 {
    let mut group_badge_letter_value = 0;

    for rucksacks in rucksack_list.chunks(3) {
        let ruck_sack_group = RucksackGroup::new(rucksacks);

        let common_badge_letter = ruck_sack_group.get_common_badge_letter();

        let letter_value = match common_badge_letter {
            Some(letter) => get_letter_value(letter),
            None => break
        };

        group_badge_letter_value += letter_value;
    }

    group_badge_letter_value
}

fn part1_task_result(rucksack_list: &Vec<Rucksack>) -> i32 {
    let mut letter_value_sum = 0;

    for rucksack in rucksack_list {
        let reeapeting_letter = rucksack.find_reapeating_letter();

        let letter_value = match reeapeting_letter {
            Some(letter) => get_letter_value(letter),
            None => break
        };

        letter_value_sum += letter_value;
    }

    letter_value_sum
}

fn get_letter_value(letter: char) -> i32 {
    let pos = LETTERS.chars().position(|x| x == letter).unwrap();
    pos as i32 + 1 // index starts at zero
}

