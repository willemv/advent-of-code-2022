#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;

    use itertools::Itertools;
    use regex::Regex;

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
}
