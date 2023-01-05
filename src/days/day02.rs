const INPUT: &str = include_str!("day02/input.txt");

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum StrategyMode {
    Normal,
    FromOutcome,
}

#[derive(Debug, Copy, Clone)]
struct Match {
    opponent: Move,
    player: Move,
    strategy_mode: StrategyMode,
}

impl Match {
    pub fn new(opponent: Move, player: Move) -> Self {
        Self {
            opponent,
            player,
            strategy_mode: StrategyMode::Normal,
        }
    }

    pub fn set_strategy_mode(&mut self, mode: StrategyMode) {
        self.strategy_mode = mode;
    }

    pub fn outcome(&self) -> MatchOutcome {
        match (self.opponent, self.player) {
            (Move::Rock, Move::Paper) => MatchOutcome::Win,
            (Move::Rock, Move::Scissors) => MatchOutcome::Loss,
            (Move::Paper, Move::Rock) => MatchOutcome::Loss,
            (Move::Paper, Move::Scissors) => MatchOutcome::Win,
            (Move::Scissors, Move::Rock) => MatchOutcome::Win,
            (Move::Scissors, Move::Paper) => MatchOutcome::Loss,
            (_, _) => MatchOutcome::Draw,
        }
    }

    pub fn determine_player_move(&self, outcome: MatchOutcome) -> Move {
        match (self.opponent, outcome) {
            (Move::Rock, MatchOutcome::Loss) => Move::Scissors,
            (Move::Rock, MatchOutcome::Win) => Move::Paper,
            (Move::Paper, MatchOutcome::Loss) => Move::Rock,
            (Move::Paper, MatchOutcome::Win) => Move::Scissors,
            (Move::Scissors, MatchOutcome::Loss) => Move::Paper,
            (Move::Scissors, MatchOutcome::Win) => Move::Rock,
            (m, MatchOutcome::Draw) => m,
        }
    }

    pub fn run(&mut self) -> u32 {
        match self.strategy_mode {
            StrategyMode::Normal => self.outcome().score() + self.player.score(),
            StrategyMode::FromOutcome => {
                let wanted_outcome = MatchOutcome::from_player_move(self.player);
                wanted_outcome.score() + self.determine_player_move(wanted_outcome).score()
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MatchOutcome {
    Loss,
    Draw,
    Win,
}

impl MatchOutcome {
    pub fn from_player_move(m: Move) -> Self {
        match m {
            Move::Rock => Self::Loss,
            Move::Paper => Self::Draw,
            Move::Scissors => Self::Win,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Move {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown move {s}"),
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct GameParser {
    moves: Vec<Match>,
}

impl GameParser {
    pub fn from_str(text: &str) -> Self {
        let moves = text
            .lines()
            .map(|x| x.split_whitespace().map(Move::from_str).collect::<Vec<_>>())
            .map(|moves| (Match::new(moves[0], moves[1])))
            .collect();

        Self { moves }
    }

    pub fn total_score_from_outcome(self) -> u32 {
        self.moves
            .into_iter()
            .map(|mut x| {
                x.set_strategy_mode(StrategyMode::FromOutcome);
                x.run()
            })
            .sum()
    }

    pub fn total_score(self) -> u32 {
        self.moves.into_iter().map(|mut x| x.run()).sum()
    }
}

pub fn run_part1() -> String {
    GameParser::from_str(INPUT).total_score().to_string()
}

pub fn run_part2() -> String {
    GameParser::from_str(INPUT)
        .total_score_from_outcome()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::{run_part1, run_part2, GameParser};

    const SAMPLE: &str = include_str!("day02/sample.txt");

    #[test]
    fn test_sample() {
        let moves = GameParser::from_str(SAMPLE);
        assert_eq!(moves.total_score(), 15);
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part1(), "12679");
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2(), "14470");
    }
}
