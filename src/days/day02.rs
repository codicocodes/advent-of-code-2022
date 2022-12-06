
pub fn solve_a(input: &str) {
    let all_pairs = parse_all_hands(input);
    let result = play_all_hands(all_pairs);
    println!("{result}");
}

pub fn solve_b(input: &str) {
    let all_pairs = parse_all_hands_b(input);
    let result = play_all_hands(all_pairs);
    println!("{result}");
}

pub fn solve(input: &str) {
    solve_a(input);
    solve_b(input);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scizzors,
}

impl HandShape {
    fn value(&self) -> i32 {
        match *self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scizzors => 3,
        }
    }
    fn beats(&self) -> HandShape {
        match *self {
            HandShape::Rock => HandShape::Scizzors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scizzors => HandShape::Paper,
        }
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
enum MatchOutcome {
    Lose,
    Draw,
    Win,
}

impl MatchOutcome {
    fn value(&self) -> i32 {
        match *self {
            MatchOutcome::Lose => 0,
            MatchOutcome::Draw => 3,
            MatchOutcome::Win => 6,
        }
    }
}

fn get_match_outcome(my_hand: HandShape, opponents_hand: HandShape) -> MatchOutcome {
    let hand_i_beat = my_hand.beats();
    let hand_opponent_beats = opponents_hand.beats();
    match (hand_i_beat, hand_opponent_beats) {
        _ if hand_i_beat == opponents_hand => MatchOutcome::Win,
        _ if hand_opponent_beats == my_hand => MatchOutcome::Lose,
        _ => MatchOutcome::Draw
    }
}

fn play_hand(my_hand: HandShape, opponents_hand: HandShape) -> i32 {
    let outcome = get_match_outcome(my_hand, opponents_hand);
    return my_hand.value() + outcome.value();
}

fn play_all_hands(hand_pairs: Vec<(HandShape, HandShape)>) -> i32 {
    let mut result = 0;
    for pair in hand_pairs {
        result += play_hand(pair.0, pair.1)
    }
    result
}

fn get_hand(code: &str) -> HandShape {
    match code {
        "A" => HandShape::Rock,
        "B" => HandShape::Paper,
        "C" => HandShape::Scizzors,
        "X" => HandShape::Rock,
        "Y" => HandShape::Paper,
        "Z" => HandShape::Scizzors,
        _ => { unimplemented!() }
    }
}

fn get_expected_result(code: &str) -> MatchOutcome {
    match code {
        "X" => MatchOutcome::Lose,
        "Y" => MatchOutcome::Draw,
        "Z" => MatchOutcome::Win,
        _ => { unimplemented!() }
    }
}

fn get_hand_from_expected_result(opponents_hand: HandShape, outcome: MatchOutcome) -> HandShape {
    match outcome {
        MatchOutcome::Lose => opponents_hand.beats(),
        MatchOutcome::Draw  => opponents_hand,
        MatchOutcome::Win => opponents_hand.beats().beats(),
    }
}

fn parse_all_hands(input: &str) -> Vec<(HandShape, HandShape)> {
    let games = str::trim(input).split("\n");
    let mut all_hands: Vec<(HandShape, HandShape)> = vec![];
    for game in games {
        let mut raw_hands = str::trim(game).split(" ");
        let (opponent_code, my_code) = (raw_hands.next().unwrap(), raw_hands.next().unwrap());
        all_hands.push(
            (get_hand(my_code), get_hand(opponent_code))
        )
    }
    all_hands
}

fn parse_all_hands_b(input: &str) -> Vec<(HandShape, HandShape)> {
    let games = str::trim(input).split("\n");
    let mut all_hands: Vec<(HandShape, HandShape)> = vec![];
    for game in games {
        let mut raw_hands = str::trim(game).split(" ");
        let (opponent_code, my_code) = (raw_hands.next().unwrap(), raw_hands.next().unwrap());
        let opponents_hand = get_hand(opponent_code);
        let my_hand = get_hand_from_expected_result(opponents_hand, get_expected_result(my_code));
        all_hands.push(
            (my_hand, opponents_hand)
        )
    }
    all_hands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draws() {
        assert_eq!(get_match_outcome(HandShape::Rock, HandShape::Rock), MatchOutcome::Draw);
        assert_eq!(get_match_outcome(HandShape::Paper, HandShape::Paper), MatchOutcome::Draw);
        assert_eq!(get_match_outcome(HandShape::Scizzors, HandShape::Scizzors), MatchOutcome::Draw);
    }

    #[test]
    fn test_wins() {
        assert_eq!(get_match_outcome(HandShape::Rock, HandShape::Scizzors), MatchOutcome::Win);
        assert_eq!(get_match_outcome(HandShape::Paper, HandShape::Rock), MatchOutcome::Win);
        assert_eq!(get_match_outcome(HandShape::Scizzors, HandShape::Paper), MatchOutcome::Win);
    }

    #[test]
    fn test_losses() {
        assert_eq!(get_match_outcome(HandShape::Rock, HandShape::Paper), MatchOutcome::Lose);
        assert_eq!(get_match_outcome(HandShape::Paper, HandShape::Scizzors), MatchOutcome::Lose);
        assert_eq!(get_match_outcome(HandShape::Scizzors, HandShape::Rock), MatchOutcome::Lose);
    }

    #[test]
    fn test_play_hand() {
        assert_eq!(play_hand(HandShape::Paper, HandShape::Rock), 8);
        assert_eq!(play_hand(HandShape::Rock, HandShape::Paper), 1);
        assert_eq!(play_hand(HandShape::Scizzors, HandShape::Scizzors), 6);
    }

    #[test]
    fn test_play_all_hands() {
        let all_hands = vec![
            (HandShape::Paper, HandShape::Rock),
            (HandShape::Rock, HandShape::Paper),
            (HandShape::Scizzors, HandShape::Scizzors),
        ];
        assert_eq!(play_all_hands(all_hands), 15);
    }

    #[test]
    fn test_solve_a() {
        let input = "A Y
B X
C Z";

        let all_pairs = parse_all_hands(input);
        let result = play_all_hands(all_pairs);
        assert_eq!(result, 15)
    }

    #[test]
    fn test_solve_b() {
        let input = "A Y
B X
C Z";

        let all_pairs = parse_all_hands_b(input);
        let result = play_all_hands(all_pairs);
        assert_eq!(result, 12)
    }
}
