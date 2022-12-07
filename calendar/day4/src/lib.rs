pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("4");

    let assigned_section_pairs = prepare_data(input);

    let fully_overlaping_pair_count = get_fully_overlaping_pairs_count(&assigned_section_pairs);
    let partially_overlaping_pair_count = get_partially_overlaping_pairs_count(assigned_section_pairs);

    (fully_overlaping_pair_count.to_string(), partially_overlaping_pair_count.to_string())
}

fn get_fully_overlaping_pairs_count(pairs:  &Vec<AssignedSectionPair>) -> i32 {
    let mut count = 0;

    for pair in pairs {
        if pair.contains_fully_overlapping_section() {
            count += 1;
        }
    }

    count
}

fn get_partially_overlaping_pairs_count(pairs: Vec<AssignedSectionPair>) -> i32 {
    let mut count = 0;

    for pair in pairs {
        if pair.contains_partially_overlapping_section() {
            count += 1;
        }
    }

    count
}

fn prepare_data(input: String) -> Vec<AssignedSectionPair> {

    let mut assigned_section_pairs: Vec<AssignedSectionPair> = Vec::new();

    for line in input.split('\n') {
        let sections: Vec<&str> = line.split(',').collect();

        assigned_section_pairs.push(AssignedSectionPair::new(sections));
    }

    assigned_section_pairs
}

struct AssignedSectionPair {
    section_one: Section,
    section_two: Section
}

impl AssignedSectionPair {
    fn new(input: Vec<&str>) -> Self {

        if input.len() != 2 {
            panic!("bad data")
        }

        let section_one = Section::new(input[0]);
        let section_two = Section::new(input[1]);
    
        AssignedSectionPair { section_one, section_two }
    }

    pub fn contains_fully_overlapping_section(&self) -> bool {
        if self.section_one.section_fully_overlaps(&self.section_two) || self.section_two.section_fully_overlaps(&self.section_one) {
            return true;
        }

        false
    }

    pub fn contains_partially_overlapping_section(&self) -> bool {
        if self.section_one.section_partially_overlaps(&self.section_two) ||
            self.section_two.section_partially_overlaps(&self.section_one) {
            return true;
        }

        false
    }

}

struct Section {
    from: i32,
    to: i32
}

impl Section {
    fn new(input: &str) -> Section {
        let numbers: Vec<&str> = input.split('-').collect();

        Section { from: numbers[0].parse::<i32>().unwrap(), to: numbers[1].parse::<i32>().unwrap() }
    }

    pub fn section_fully_overlaps(&self, other: &Section) -> bool {
        if self.from >= other.from && self.to <= other.to {
            return true;
        }

        false
    }

    pub fn section_partially_overlaps(&self, other: &Section) -> bool {
        if self.from >= other.from && self.from <= other.to {
            return true;
        }

        if self.to >= other.from && self.to <= other.to {
            return true;
        }

        false
    }
}