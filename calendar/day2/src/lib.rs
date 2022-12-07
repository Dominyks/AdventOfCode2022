mod player_hands;
mod score_calculation;

pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("2");

    let score_part_1 = score_calculation::calculate_score_part1(&input);
    let score_part_2 = score_calculation::calculate_score_part2(&input);

    (score_part_1.to_string(), score_part_2.to_string())
}