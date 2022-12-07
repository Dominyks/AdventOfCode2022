use crate::rucksack::Rucksack;

pub fn convert_input_to_rucksakcs(input: String) -> Vec<Rucksack> {
    let mut rucksack_list: Vec<Rucksack> = Vec::new();

    for line in  input.split('\n') {
        let rucksack = Rucksack::new(line.to_string());
        rucksack_list.push(rucksack);
    }

    rucksack_list
}