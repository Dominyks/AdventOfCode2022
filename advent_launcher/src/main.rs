fn main() {
    let variables: Vec<String> = std::env::args().collect();

    if variables.len() <= 1 {
        println!("Please provide one argument (the days number of which you would like to see solution). E.x. 1");
        return ();
    }

    let day_number = variables.get(1);
    let day_number = match day_number {
        Some(value) => value,
        None => return ()
    };

    let answers = get_specified_day_answers(day_number.as_str());

    print_answers(answers.0, answers.1);
}

fn print_answers(marker_size: String, marker_start: String) {
    println!("****************************");
    println!("part1 answer: {:?}", marker_size);
    println!("part2 answer: {:?}", marker_start);
    println!("****************************");
}

fn get_specified_day_answers(day_number: &str) -> (String, String) {
    match day_number {
        "1" => day1::get_answers(),
        "2" => day2::get_answers(),
        "3" => day3::get_answers(),
        "4" => day4::get_answers(),
        "5" => day5::get_answers(),
        "6" => day6::get_answers(),
        _ => panic!("Day: {:?} is not yet prepared or it is too soon ;)", day_number)
    }
}