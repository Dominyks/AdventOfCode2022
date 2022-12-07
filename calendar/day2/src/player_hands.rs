pub struct PlayerHandPair {
    your_hand_symbol: String,
    your_hand: RockPaperScissors,
    _oponnent_hand_symbol: String,
    oponnent_hand: RockPaperScissors,
}

impl PlayerHandPair {
    pub fn new(
        your_hand_symbol: String,
        your_hand: RockPaperScissors,
        _oponnent_hand_symbol: String,
        oponnent_hand: RockPaperScissors
    ) -> Self {
        PlayerHandPair { 
            your_hand_symbol,
            your_hand,
            _oponnent_hand_symbol,
            oponnent_hand
        }
    }

    pub fn calculate_hand_score_part1(&self) -> i32 {
        let hand_outcome = self.calculate_hand_outcome();

        hand_outcome.result_score() + self.your_hand.value()
    }

    pub fn calculate_hand_score_part2(&self) -> i32 {
        let desired_game_outcome = HandResult::desired_outcome(&self.your_hand_symbol);

        let hand_to_play = PlayerHandPair::get_hand_to_play(&desired_game_outcome, &self.oponnent_hand);

        desired_game_outcome.result_score() + hand_to_play.value()
    }

    fn get_hand_to_play(desired_outcome: &HandResult, oponent_hand: &RockPaperScissors) -> RockPaperScissors {
        if matches!(desired_outcome, HandResult::Win) {
            if matches!(oponent_hand, RockPaperScissors::Rock) {
                return RockPaperScissors::Paper;
            }

            if matches!(oponent_hand, RockPaperScissors::Paper) {
                return RockPaperScissors::Scissors;
            }

            if matches!(oponent_hand, RockPaperScissors::Scissors) {
                return RockPaperScissors::Rock;
            }
        }

        if matches!(desired_outcome, HandResult::Lose) {
            if matches!(oponent_hand, RockPaperScissors::Rock) {
                return RockPaperScissors::Scissors;
            }

            if matches!(oponent_hand, RockPaperScissors::Paper) {
                return RockPaperScissors::Rock;
            }

            if matches!(oponent_hand, RockPaperScissors::Scissors) {
                return RockPaperScissors::Paper;
            }
        }

        if matches!(desired_outcome, HandResult::Draw) {
            if matches!(oponent_hand, RockPaperScissors::Rock) {
                return RockPaperScissors::Rock;
            }

            if matches!(oponent_hand, RockPaperScissors::Paper) {
                return RockPaperScissors::Paper;
            }

            if matches!(oponent_hand, RockPaperScissors::Scissors) {
                return RockPaperScissors::Scissors;
            }
        }
        panic!("cant happen");
    }

    fn calculate_hand_outcome(&self) -> HandResult {

        if  matches!(&self.your_hand, RockPaperScissors::Rock) && matches!(&self.oponnent_hand, RockPaperScissors::Rock) {
           return HandResult::Draw;
        }
        if  matches!(&self.your_hand, RockPaperScissors::Paper) && matches!(&self.oponnent_hand, RockPaperScissors::Paper) {
            return HandResult::Draw;
        }
        if  matches!(&self.your_hand, RockPaperScissors::Scissors) && matches!(&self.oponnent_hand, RockPaperScissors::Scissors) {
            return HandResult::Draw;
        }


        if  matches!(&self.your_hand, RockPaperScissors::Rock) && matches!(&self.oponnent_hand, RockPaperScissors::Scissors) {
            return HandResult::Win;
        }
        if  matches!(&self.your_hand, RockPaperScissors::Rock) && matches!(&self.oponnent_hand, RockPaperScissors::Paper) {
            return HandResult::Lose;
        }


        if  matches!(&self.your_hand, RockPaperScissors::Paper) && matches!(&self.oponnent_hand, RockPaperScissors::Rock) {
            return HandResult::Win;
        }
        if  matches!(&self.your_hand, RockPaperScissors::Paper) && matches!(&self.oponnent_hand, RockPaperScissors::Scissors) {
            return HandResult::Lose;
        }


        if  matches!(&self.your_hand, RockPaperScissors::Scissors) && matches!(&self.oponnent_hand, RockPaperScissors::Paper) {
            return HandResult::Win;
        }
        if  matches!(&self.your_hand, RockPaperScissors::Scissors) && matches!(&self.oponnent_hand, RockPaperScissors::Rock) {
            return HandResult::Lose;
        }

        panic!("invalid scenario");
    }
}

pub fn parse_player_hand(symbol: &str) -> RockPaperScissors {
    match symbol {
        "A" => RockPaperScissors::Rock,
        "X" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "Y" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        "Z" => RockPaperScissors::Scissors,
        _ => panic!("symbol not matched")
    }
}

pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn value(&self) -> i32 {
        match *self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3
        }
    }
}

pub enum HandResult {
    Win,
    Lose,
    Draw,
}

impl HandResult {
    fn result_score(&self) -> i32 {
        match *self {
            HandResult::Win => 6,
            HandResult::Draw => 3,
            HandResult::Lose => 0,
        }
    }

    fn desired_outcome(hint: &str) -> HandResult {
        match hint {
            "X" => HandResult::Lose,
            "Y" => HandResult::Draw,
            "Z" => HandResult::Win,
            _ => panic!("no scenario for value given")
        }
    }
}