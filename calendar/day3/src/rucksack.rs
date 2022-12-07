pub struct Rucksack 
{
    supplies: String,
    split_at: usize
}

pub struct RucksackGroup<'a>
{
    rucksacks: &'a [Rucksack]
}

impl<'a> RucksackGroup<'a>
{
    pub fn new(rucksacks: &'a [Rucksack]) -> Self
    {
        if rucksacks.len() != 3 {
            panic!("bad group size")
        }

        RucksackGroup { rucksacks }
    }

    pub fn get_common_badge_letter(self) -> Option<char> {

        let (first_rucksack, rest_of_rucksacks) = self.rucksacks.split_at(1);

        let letters = first_rucksack[0].supplies.chars();

        for letter in letters {
            let mut count_left_to_match = 2;

            for rucksack in rest_of_rucksacks {
                if rucksack.supplies.contains(letter) {
                    count_left_to_match -= 1;
                }
            }

            if count_left_to_match == 0 {
                return Some(letter);
            }
        }


        // for rucksack in self.rucksacks {

        //     let rucksack_letters = rucksack.supplies.chars();
        //     let mut count_to_match = 0;

        //     for rest_of_rucksack in self.rucksacks.iter().filter(|&x| !std::ptr::eq(x, rucksack)) {
        //         let other_letters = rest_of_rucksack.supplies.chars();

        //         for letter1 in rucksack_letters {
        //             for letter2 in other_letters {
        //                 if letter1 == letter2 {
        //                     count_to_match += 1;
        //                 }
        //             }
        //         }

        //     }
        // }

        None
    }
}

impl Rucksack {
    pub fn new(supplies: String) -> Self
    {
        let supplies_size = supplies.chars().count();

        let size_divided = supplies_size % 2;

        if size_divided != 0 {
            panic!("supplies must be of even count")
        }

        let split_at = supplies_size / 2;
        Rucksack { supplies, split_at }
    }

    pub fn find_reapeating_letter(&self) -> Option<char> {
        let (comparment_1, compartment_2) = self.get_compartments();

        for compartment_1_letter in comparment_1.chars()
        {
            for compartment_2_letter in compartment_2.chars()
            {
                if compartment_1_letter == compartment_2_letter {
                    return Option::Some(compartment_1_letter);
                }
            }
        }

        None
    }

    fn get_compartments(&self) -> (&str, &str)
    {
        self.supplies.split_at(self.split_at)
    }
}