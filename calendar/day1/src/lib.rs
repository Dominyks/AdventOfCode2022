pub fn get_answers() -> (String, String) {
    let input = input_reader::get_input("1");

    let elf_calorie_list = group_elf_calories(&input);
    let mut sum_cal_vec = get_summed_calories_list(elf_calorie_list);

    sum_cal_vec.sort_by(|a, b| b.cmp(a));

    let top1 = sum_cal_vec[0];
    let top_3_sum = sum_cal_vec[0] + sum_cal_vec[1] + sum_cal_vec[2];

    (top1.to_string(), top_3_sum.to_string())
}

fn group_elf_calories(input: &String) -> Vec<Vec<i32>> {
    let mut elf_calorie_list: Vec<Vec<i32>> = Vec::new();

    let contents_split = input.split("\n\n");
    let lines: Vec<&str> = contents_split.collect();

    for line in lines {
        let calories: Vec<&str> = line.split("\n").collect();

        let mut int_cal_vec: Vec<i32> = Vec::new();
        for cal in calories {
            if !cal.is_empty()
            {
                let cal_int = cal.parse::<i32>().unwrap();
                int_cal_vec.push(cal_int);
            }
        }

        elf_calorie_list.push(int_cal_vec);
    }

    elf_calorie_list
}

fn get_summed_calories_list(elf_calorie_list: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sum_cal_vec: Vec<i32> = Vec::new();

    for cals in elf_calorie_list{
        let mut cal_max = 0;

        for cal in cals {
            cal_max += cal;
        }

        sum_cal_vec.push(cal_max);
    }

    sum_cal_vec
}