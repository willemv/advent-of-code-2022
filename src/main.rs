fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::collections::HashSet;
    use std::env;
    use std::env::VarError;
    use std::error::Error;
    use std::ops::RangeInclusive;

    use itertools::Itertools;
    use regex::Regex;
    use reqwest;
    use transpose::transpose;

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

            let common_chars: Vec<char> = first_chars_set
                .intersection(&second_chars_set)
                .copied()
                .collect();
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

    #[test]
    fn day4() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(4)?;

        fn parse_range(input: &str) -> std::ops::RangeInclusive<u32> {
            let parts = input.split("-").collect_vec();
            let start = parts[0].parse::<u32>().unwrap();
            let end = parts[1].parse::<u32>().unwrap();

            start..=end
        }

        fn covers(first: &RangeInclusive<u32>, second: &RangeInclusive<u32>) -> bool {
            second.start() >= first.start() && second.end() <= first.end()
        }

        fn overlaps(first: &RangeInclusive<u32>, second: &RangeInclusive<u32>) -> bool {
            !(first.start() > second.end() || first.end() < second.start())
        }

        let mut num_covering = 0;
        let mut num_intersecting = 0;

        for line in input.lines() {
            let groups = line.split(",").collect_vec();
            let first_range = parse_range(groups[0]);
            let second_range = parse_range(groups[1]);

            if covers(&first_range, &second_range) || covers(&second_range, &first_range) {
                num_covering += 1;
            }

            if overlaps(&first_range, &second_range) {
                num_intersecting += 1;
            }
        }

        println!("Number of fully covering ranges: {}", num_covering);
        println!("Number of intersecting ranges: {num_intersecting}");

        Ok(())
    }

    #[derive(Clone, Debug)]
    struct Depot {
        stacks: [Vec<char>; 9],
    }

    enum Day5Mode {
        OneByOne,
        AllAtOnce,
    }

    #[test]
    fn day5_part1() -> Result<(), Box<dyn std::error::Error>> {
        day5(Day5Mode::OneByOne)
    }

    #[test]
    fn day5_part2() -> Result<(), Box<dyn std::error::Error>> {
        day5(Day5Mode::AllAtOnce)
    }

    fn day5(mode: Day5Mode) -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(5)?;

        let mut depot = Depot {
            stacks: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        };

        let mut lines = input.lines().into_iter();

        // process the header of the input until we encounter an empty line
        loop {
            let line = lines.next().unwrap();

            if line.starts_with(" 1") {
                continue;
            }
            if line.is_empty() {
                break;
            }

            for i in 0..9 {
                if let Some(c) = line.chars().nth(1 + (4 * i)) {
                    if c != ' ' {
                        depot.stacks[i].insert(0, c);
                    }
                }
            }
        }

        // process the remainder of the lines
        let r = Regex::new("move (\\d+) from (\\d+) to (\\d+)")?;
        for line in lines {
            let c = r.captures(&line).unwrap();
            let count = c[1].parse::<usize>()?;
            let from = c[2].parse::<usize>()? - 1;
            let to = c[3].parse::<usize>()? - 1;

            match mode {
                Day5Mode::OneByOne => {
                    for _ in 0..count {
                        let popped = depot.stacks[from].pop().unwrap();
                        depot.stacks[to].push(popped);
                    }
                }
                Day5Mode::AllAtOnce => {
                    let from = &mut depot.stacks[from];
                    let mut drained = from.drain((from.len() - count)..).collect_vec();
                    depot.stacks[to].append(&mut drained);
                }
            }
        }

        let mut answer = String::new();
        for mut stack in depot.stacks {
            if !stack.is_empty() {
                answer.push(stack.pop().unwrap());
            }
        }
        println!("Answer: {answer}");
        Ok(())
    }

    #[test]
    fn day6_part1() -> Result<(), Box<dyn std::error::Error>> {
        day6(4)
    }

    #[test]
    fn day6_part2() -> Result<(), Box<dyn std::error::Error>> {
        day6(14)
    }

    fn day6(num_chars: usize) -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(6)?;
        let mut chars = Vec::with_capacity(num_chars);
        for (index, char) in input.chars().enumerate() {
            if chars.len() < num_chars {
                chars.push(char);
                continue;
            }

            chars.remove(0);
            chars.push(char);

            assert!(chars.len() == num_chars);

            let unique_chars: HashSet<&char> = chars.iter().collect();

            if unique_chars.len() == num_chars {
                println!("Set: {:?}", unique_chars);
                println!("Number of characters to receive: {}", index + 1);
                return Ok(());
            }
        }
        panic!(
            "Didn't find a sequence of {} subsequent different characters",
            num_chars
        );
    }

    #[test]
    fn day7() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(7)?;

        fn traverse<F>(input: &str, mut finish_dir: F) -> Result<usize, Box<dyn std::error::Error>>
        where
            F: FnMut(usize),
        {
            let mut sizes_stack: Vec<usize> = Vec::new();
            let mut total_size = None;
            for line in input.lines() {
                if line.eq("$ cd ..") {
                    let dir_size = sizes_stack.pop().unwrap();
                    finish_dir(dir_size);
                    *(sizes_stack.last_mut().unwrap()) += dir_size; //add the size of a child dir to the current dir
                } else if line.starts_with("$ cd /") {
                    assert!(sizes_stack.is_empty());
                    sizes_stack.push(0);
                } else if line.starts_with("$ cd ") {
                    sizes_stack.push(0);
                } else if line.starts_with("$ ls") {
                    //ignore
                } else if line.starts_with("dir ") {
                    //ignore, we'll get there later
                } else {
                    // line with a size and filename
                    let space = line.find(' ').unwrap();
                    let file_size: usize = (&line[..space]).parse()?;
                    *(sizes_stack.last_mut().unwrap()) += file_size;
                }
            }

            while let Some(dir_size) = sizes_stack.pop() {
                finish_dir(dir_size);
                if let Some(r) = sizes_stack.last_mut() {
                    *r += dir_size;
                } else {
                    println!("Total size: {dir_size}");
                    total_size = Some(dir_size);
                }
            }
            Ok(total_size.unwrap())
        }

        let total_size = {
            let max_size = 100_000;
            let mut sum = 0;
            let total_size = traverse(&input, |dir_size| {
                if dir_size <= max_size {
                    sum += dir_size;
                }
            })?;

            println!("sum of filtered dirs: {sum}");
            total_size
        };

        {
            let space_to_free_up = 30_000_000 - (70_000_000 - total_size);
            let mut size_of_dir_to_delete = usize::MAX;

            traverse(&input, |dir_size| {
                if dir_size >= space_to_free_up && dir_size < size_of_dir_to_delete {
                    size_of_dir_to_delete = dir_size;
                }
            })?;
            println!("size of dir to delete: {size_of_dir_to_delete}");
        }

        Ok(())
    }

    #[test]
    fn day8() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(8)?;

        let mut width = None;
        let mut height = 0;
        let mut matrix: Vec<usize> = vec![];
        for line in input.lines() {
            height += 1;
            if width == None {
                width = Some(line.len());
            }
            for char in line.chars() {
                matrix.push(char.to_string().parse().unwrap());
            }
        }

        let width = width.unwrap();
        fn collect_visible_trees(matrix: &[usize], width: usize) -> HashSet<(usize, usize)> {
            let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
            let mut max_tree_height = None;
            for (i, tree_height) in matrix.iter().enumerate() {
                let (row, column) = (i / width, i % width);
                if column == 0 {
                    max_tree_height = None;
                }

                let coord = (row, column);
                if let Some(height) = max_tree_height {
                    if tree_height > height {
                        visible_trees.insert(coord);
                        max_tree_height = Some(tree_height);
                    }
                } else {
                    visible_trees.insert(coord);
                    max_tree_height = Some(tree_height);
                }
            }

            max_tree_height = None;

            for (j, tree_height) in matrix.iter().rev().enumerate() {
                let i = matrix.len() - 1 - j;
                let (row, column) = (i / width, i % width);

                if column == width - 1 {
                    max_tree_height = None;
                }

                let coord = (row, column);
                if let Some(current_max_height) = max_tree_height {
                    if tree_height > current_max_height {
                        visible_trees.insert(coord);
                        max_tree_height = Some(tree_height);
                    }
                } else {
                    visible_trees.insert(coord);
                    max_tree_height = Some(tree_height);
                }
            }
            visible_trees
        }

        let mut visible_trees = collect_visible_trees(&matrix, width);
        let mut transposed = vec![0; matrix.len()];

        transpose(&matrix, &mut transposed, width, height);
        let transposed_visible_trees = collect_visible_trees(&transposed, height);
        visible_trees.extend(transposed_visible_trees.into_iter().map(|(x, y)| (y, x)));

        println!(
            "Number of trees visible from the outside: {}",
            visible_trees.len()
        );

        fn scenic_score(
            x: usize,
            y: usize,
            forest: &[usize],
            forest_width: usize,
            forest_height: usize,
        ) -> (usize,usize,usize,usize) {
            let tree_index = (y * forest_width) + x;
            let tree_height = forest[tree_index];

            let mut score_west = 0;
            {
                let mut i = tree_index;
                while i > y * forest_width {
                    score_west += 1;
                    i -= 1; //cannot underflow because of condition above;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }

            let mut score_north = 0;
            {
                let mut i = tree_index;
                while i > forest_width {
                    score_north += 1;
                    i -= forest_width; //up a row (cannot underflow because of while condition)
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }
            let mut score_east = 0;
            {
                let mut i = tree_index;
                let end_of_row = ((y + 1) * forest_width) - 1;
                while i < end_of_row {
                    score_east += 1;
                    i += 1;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }
            let mut score_south = 0;
            {
                let mut i = tree_index;
                let end_of_col = (forest_width * (forest_height - 1)) + x;
                while i < end_of_col {
                    score_south += 1;
                    i += forest_width;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }

            (score_north , score_east , score_south , score_west)
        }

        let mut max_score = 0;
        for y in 0..height {
            for x in 0..width {
                let scores = scenic_score(x, y, &matrix, width, height);
                let score = scores.0 * scores.1 * scores.2 * scores.3;
                if score >= max_score {
                    println!("new highscore for ({x}, {y}): {score} ({scores:?})");
                    max_score = score;
                }
            }
        }

        Ok(())
    }
}
