fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    extern crate reqwest;

    use std::cmp::Reverse;
    use std::collections::HashSet;
    use std::env;
    use std::env::VarError;
    use std::error::Error;

    use itertools::Itertools;

    fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();

        let session_id = env::var("AOC_SESSION_ID")?;
        if session_id.is_empty() {
            Err(VarError::NotPresent)?
        }

        let body = client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("Cookie", format!("session={}", session_id))
            .send()?
            .text()?;

        Ok(body)
    }

    #[test]
    fn day1() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(1)?;
        let mut calories: Vec<u32> = Vec::new();
        for section in input.split("\n\n") {
            let mut current: u32 = 0;
            for line in section.lines() {
                current = current + line.parse::<u32>()?;
            }
            calories.push(current);
        }
        calories.sort_by_key(|w| Reverse(*w));

        println!("Max calories: {}", calories[0]);
        println!(
            "Sum of calories of elves carrying most calories: {}",
            calories.into_iter().take(3).sum::<u32>()
        );
        Ok(())
    }

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

    fn day3_priority(c: char) -> u32 {
        if c.is_ascii_lowercase() {
            (c as u32) - 97 + 1 //a is ascii code 97
        } else if c.is_ascii_uppercase() {
            (c as u32) - 65 + 27 // A is ascii code 65
        } else {
            panic!("unsupported char");
        }
    }

    #[test]
    fn day3_part1() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(3)?;
        let mut priority_sum = 0;
        for line in input.lines() {
            let all_chars = line.chars().collect_vec();
            let mut first_chars_set: HashSet<char> = HashSet::new();
            let mut second_chars_set: HashSet<char> = HashSet::new();
            for char in &all_chars[0..all_chars.len() / 2] {
                first_chars_set.insert(*char);
            }

            for char in &all_chars[all_chars.len() / 2..] {
                second_chars_set.insert(*char);
            }

            let common_chars: Vec<char> =
                first_chars_set.intersection(&second_chars_set).copied().collect();
            let common_char = common_chars[0];
            let priority = day3_priority(common_char);
            priority_sum += priority;
        }
        println!("Total priority: {}", priority_sum);
        Ok(())
    }

    #[test]
    fn day3_part2() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(3)?;
        let mut priority_sum = 0;
        for line_triple_iter in &input.lines().chunks(3) {
            let line_triple = line_triple_iter.collect_vec();
            let mut first_chars_set: HashSet<char> = HashSet::new();
            let mut second_chars_set: HashSet<char> = HashSet::new();
            let mut third_chars_set: HashSet<char> = HashSet::new();
            for c in line_triple[0].chars().collect_vec() {
                first_chars_set.insert(c);
            }
            for c in line_triple[1].chars().collect_vec() {
                second_chars_set.insert(c);
            }
            for c in line_triple[2].chars().collect_vec() {
                third_chars_set.insert(c);
            }

            let first_intersection: HashSet<char> = first_chars_set
                .intersection(&second_chars_set)
                .copied()
                .collect();
            let common_chars: Vec<char> = first_intersection
                .intersection(&third_chars_set)
                .copied()
                .collect();

            let common_char = common_chars[0];
            let priority = day3_priority(common_char);
            priority_sum += priority;
        }
        println!("Total priority: {}", priority_sum);
        Ok(())
    }
}
