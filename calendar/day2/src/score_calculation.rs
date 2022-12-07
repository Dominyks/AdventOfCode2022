pub fn calculate_score_part1(input: &String) -> i32 {
    let all_hands = prepare_data::prepare_data(input);

    let mut score_sum = 0;

    for hand in all_hands {
        let hand_score = hand.calculate_hand_score_part1();
        score_sum += hand_score;
    }

    score_sum
}

pub fn calculate_score_part2(input: &String) -> i32 {
    let all_hands = prepare_data::prepare_data(input);

    let mut score_sum = 0;

    for hand in all_hands {
        let hand_score = hand.calculate_hand_score_part2();
        score_sum += hand_score;
    }

    score_sum
}

mod prepare_data{
    use crate::player_hands::{PlayerHandPair, parse_player_hand};

    pub fn prepare_data(data: &String) -> Vec<PlayerHandPair> {
        let mut all_hands: Vec<PlayerHandPair> = Vec::new();
        
        let lines: Vec<&str> = data.split('\n').collect();

        for line in lines {

            if line.is_empty() {
                break;
            }

            let hands: Vec<&str> = line.split(' ').collect();

            if hands.len() > 2 {
                panic!("hand pair cannot contain more than 2 elements");
            }

            let oponnent_hand_symbol = hands[0];
            let your_hand_symbol = hands[1];

            let oponnent_hand = parse_player_hand(oponnent_hand_symbol);
            let your_hand = parse_player_hand(your_hand_symbol);

            let game_pair = PlayerHandPair::new(
                your_hand_symbol.to_string(),
                your_hand,
                oponnent_hand_symbol.to_string(),
                oponnent_hand
            );

            all_hands.push(game_pair);
        }

        all_hands
    }
}