#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;

    #[derive(Clone, Copy, Debug)]
    enum Choice {
        ROCK,
        PAPER,
        SCISSORS,
    }

    impl Choice {
        fn score(&self) -> u32 {
            match &self {
                Self::ROCK => 1,
                Self::PAPER => 2,
                Self::SCISSORS => 3,
            }
        }

        fn compare(&self, other: &Choice) -> Outcome {
            match (&self, other) {
                (&Self::ROCK, &Self::ROCK) => Outcome::DRAW,
                (&Self::ROCK, &Self::SCISSORS) => Outcome::WIN,
                (&Self::ROCK, &Self::PAPER) => Outcome::LOSS,

                (&Self::PAPER, &Self::PAPER) => Outcome::DRAW,
                (&Self::PAPER, &Self::ROCK) => Outcome::WIN,
                (&Self::PAPER, &Self::SCISSORS) => Outcome::LOSS,

                (&Self::SCISSORS, &Self::SCISSORS) => Outcome::DRAW,
                (&Self::SCISSORS, &Self::ROCK) => Outcome::LOSS,
                (&Self::SCISSORS, &Self::PAPER) => Outcome::WIN,
            }
        }

        fn nemesis(&self) -> Choice {
            match &self {
                &Self::ROCK => Self::PAPER,
                &Self::PAPER => Self::SCISSORS,
                &Self::SCISSORS => Self::ROCK,
            }
        }

        fn sub(&self) -> Choice {
            match &self {
                &Self::ROCK => Self::SCISSORS,
                &Self::PAPER => Self::ROCK,
                &Self::SCISSORS => Self::PAPER,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Outcome {
        WIN,
        DRAW,
        LOSS,
    }

    impl Outcome {
        fn score(&self) -> u32 {
            match &self {
                Self::WIN => 6,
                Self::DRAW => 3,
                Self::LOSS => 0,
            }
        }
    }

    #[test]
    fn day2_part1() -> Result<(), Box<dyn std::error::Error>> {
        fn parse(c: char) -> Choice {
            match c {
                'A' | 'X' => Choice::ROCK,
                'B' | 'Y' => Choice::PAPER,
                'C' | 'Z' => Choice::SCISSORS,
                _ => panic!("Unknown character {}", c),
            }
        }

        let input = get_input(2)?;
        let mut total_score = 0;
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let opponent = parse(chars[0]);
            let choice = parse(chars[2]);

            let comparison = choice.compare(&opponent);

            let score = comparison.score() + choice.score();
            total_score += score;
        }
        println!("total score: {}", total_score);
        Ok(())
    }

    #[test]
    fn day2_part2() -> Result<(), Box<dyn std::error::Error>> {
        fn parse_opponent(c: char) -> Choice {
            match c {
                'A' => Choice::ROCK,
                'B' => Choice::PAPER,
                'C' => Choice::SCISSORS,
                _ => panic!("Unknown character {}", c),
            }
        }

        fn parse_outcome(c: char) -> Outcome {
            match c {
                'X' => Outcome::LOSS,
                'Y' => Outcome::DRAW,
                'Z' => Outcome::WIN,
                _ => panic!("Unknown character {}", c),
            }
        }

        let input = get_input(2)?;
        let mut total_score = 0;
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let opponent = parse_opponent(chars[0]);
            let expected_outcome = parse_outcome(chars[2]);

            let required_choice = match expected_outcome {
                Outcome::WIN => opponent.nemesis(),
                Outcome::DRAW => opponent,
                Outcome::LOSS => opponent.sub(),
            };

            let score = expected_outcome.score() + required_choice.score();
            total_score += score;
        }
        println!("total score: {}", total_score);
        Ok(())
    }
}
