use std::fs;

pub fn get_input(day: &str) -> String {
    let path = format!(".//calendar//input-reader//input_data//day{}_input.txt", day);

    match fs::read_to_string(path) {
        Ok(input) => input,
        Err(err) => panic!("{:?}", err)
    }
}