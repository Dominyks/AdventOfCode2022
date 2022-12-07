use std::collections::{VecDeque, LinkedList};

pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("5");

    let (crate_stacks, instructions) = prepare_data(input);

    let mut stack_for_first_message = crate_stacks.to_vec();
    let mut stack_for_second_message = crate_stacks.to_vec();

    let first_message = get_message_part1(&mut stack_for_first_message, &instructions);
    let second_message = get_message_part2(&mut stack_for_second_message, &instructions);

    (first_message, second_message)
}

fn get_message_part1(crate_stacks: &mut Vec<CrateStack>, instructions: &LinkedList<Instruction>) -> String {
    let insert_order = InsertOrder::FILO;
    get_message(crate_stacks, instructions, insert_order)
}

fn get_message_part2(crate_stacks: &mut Vec<CrateStack>, instructions: &LinkedList<Instruction>) -> String {
    let insert_order = InsertOrder::FIFO;
    get_message(crate_stacks, instructions, insert_order)
}

fn get_message(crate_stacks: &mut Vec<CrateStack>, instructions: &LinkedList<Instruction>, insert_order: InsertOrder) -> String {

    for instruction in instructions {
        let mut letters_to_move: LinkedList<char> = LinkedList::new();
        {
            let from_stack = get_crate_stack_at(crate_stacks, instruction.from_index).unwrap();
            let mut i = 0;

            while i < instruction.move_count {
                let letter = from_stack.letter_stack.pop_front().unwrap();
                if matches!(insert_order, InsertOrder::FILO) {
                    letters_to_move.push_front(letter);
                } else if matches!(insert_order, InsertOrder::FIFO) {
                    letters_to_move.push_back(letter);
                }
                
                i += 1;
            }
        }  

        let to_stack = get_crate_stack_at(crate_stacks, instruction.to_index).unwrap();

        for letter in letters_to_move {
            to_stack.letter_stack.push_front(letter);
        }
    }

    let message = get_message_from_stack(crate_stacks);

    message
}
enum InsertOrder {
    FIFO,
    FILO,
}

fn get_message_from_stack(crate_stacks: &mut Vec<CrateStack>) -> String {
    let mut message = String::new();
    for crate_stack in crate_stacks.iter_mut() {
        let letter = crate_stack.letter_stack.pop_front().unwrap();
        message.push(letter);
    }

    message
}
fn prepare_data(input: String) -> (Vec<CrateStack>, LinkedList<Instruction>) {
    let result = input.split_once("\n\n");

    let (crate_info_input, instructions_info) = match result {
        Some(x) => x,
        None => panic!("bad input data")
    };

    let crate_stacks = get_crate_stacks(crate_info_input);
    let instructions = get_instructions(instructions_info);

    (crate_stacks, instructions)
}

fn get_instructions(instructions_info: &str) -> LinkedList<Instruction> {
    let instructions_info_lines: VecDeque<&str> = instructions_info.split('\n').collect();

    let mut instruction_set: LinkedList<Instruction> = LinkedList::new();

    for instructions_info_line in instructions_info_lines {
        if instructions_info_line.is_empty() {
            break;
        }

        let mut line_info = instructions_info_line.split_ascii_whitespace();
        let mut instruction = Instruction{ move_count:0, from_index: 0, to_index: 0};

        let mut value_option = line_info.next();
        while value_option.is_some() {
            match value_option.unwrap() {
                "move" => instruction.add_move_count(line_info.next()),
                "from" => instruction.add_from_index(line_info.next()),
                "to" => instruction.add_to_index(line_info.next()),
                _ => panic!("bad instructions")
            };

            value_option = line_info.next();
        }

        instruction_set.push_back(instruction);
    }
    
    instruction_set
}



fn get_crate_stacks(crate_info_input: &str) -> Vec<CrateStack> {

    let mut crate_info_lines: Vec<&str> = crate_info_input.split('\n').collect();

    let crate_count_line = crate_info_lines.pop().unwrap();

    let crate_numbers: Vec<i32> = crate_count_line
    .split_ascii_whitespace()
    .map(|x| x.parse::<i32>().expect("not a number"))
    .collect();

    let mut crate_data_set: Vec<CrateStack> = Vec::with_capacity(crate_numbers.len());

    for number in crate_numbers {
        crate_data_set.push(CrateStack{ stack_number: number, letter_stack: LinkedList::new() });
    }

    for line in crate_info_lines {

        let mut index = 1;
        for bytes in line.as_bytes().chunks(4) {
            for byte in bytes  {
                if byte.is_ascii_alphabetic() {
                    let letter = *byte as char;

                    let a = get_crate_stack_at(&mut crate_data_set, index).unwrap();
                    a.letter_stack.push_back(letter)
                }
            }

            index += 1;
        }
    }

    crate_data_set
}

fn get_crate_stack_at(v: &mut Vec<CrateStack>, i: i32) -> Option<&mut CrateStack> {
    for elem in v {
        if elem.stack_number == i {
            return Some(elem)
        }
    }

    None
}

#[derive(Clone)]
struct CrateStack {
     stack_number: i32,
     letter_stack: LinkedList<char>
}


struct Instruction {
    move_count: i32,
    from_index: i32,
    to_index: i32
}

impl Instruction {
    pub fn add_move_count(&mut self, num_option: Option<&str>) {
        self.move_count = parse_value(num_option);
    }

    pub fn add_from_index(&mut self, num_option: Option<&str>) {
        self.from_index = parse_value(num_option);
    }

    pub fn add_to_index(&mut self, num_option: Option<&str>) {
        self.to_index = parse_value(num_option);
    }
}

fn parse_value(num_option: Option<&str>) -> i32 {
    let result = match num_option {
        Some(value) => value.parse::<i32>(),
        None => Ok(0)
    };

    match result {
        Ok(x) => x,
        Err(_) => 0
    }
}