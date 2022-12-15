#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error};

    use itertools::Itertools;

    use crate::common::tests::{get_input, get_sample_input};

    const STARTING_ITEMS_PREFIX: &str = "  Starting items: ";
    const OPERATIONS_PREFIX: &str = "  Operation: new = old ";
    const TEST_PREFIX: &str = "  Test: divisible by ";
    const POSITIVE_ACTION_PREFIX: &str = "    If true: throw to monkey ";
    const NEGATIVE_ACTION_PREFIX: &str = "    If false: throw to monkey ";

    #[derive(Clone, Copy, Debug)]
    enum Op {
        Add(usize),
        Mul(usize),
        Square,
    }

    impl Op {
        fn execute(&self, old: usize) -> usize {
            match self {
                &Op::Add(a) => old + a,
                &Op::Mul(m) => old * m,
                &Op::Square => old * old,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Test {
        Divisible(usize),
    }

    impl Test {
        fn apply(&self, worry_level: usize) -> bool {
            match self {
                &Self::Divisible(d) => worry_level % d == 0,
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Monkey {
        item_worry_levels: Vec<usize>,
        op: Op,
        test: Test,
        positive_target: usize,
        negative_target: usize,
        inspection_count: usize,
    }

    #[test]
    fn day11_part1_reference() -> Result<(), Box<dyn Error>> {
        let monkey_business_level = day11(get_sample_input(11)?, 3, 20)?;
        assert!(monkey_business_level == 10605);
        Ok(())
    }

    #[test]
    fn day11_part1() -> Result<(), Box<dyn Error>> {
        let monkey_business_level = day11(get_input(11)?, 3, 20)?;
        println!("Monkey business level: {monkey_business_level}");
        Ok(())
    }
    #[test]
    fn day11_part2_reference() -> Result<(), Box<dyn Error>> {
        let monkey_business_level = day11(get_sample_input(11)?, 1, 10_000)?;
        assert!(monkey_business_level == 2713310158);
        Ok(())
    }

    #[test]
    fn day11_part2() -> Result<(), Box<dyn Error>> {
        let monkey_business_level = day11(get_input(11)?, 1, 10_000)?;
        println!("Monkey business level: {monkey_business_level}");
        Ok(())
    }

    fn day11(
        input: String,
        worry_reduction: usize,
        number_of_rounds: usize,
    ) -> Result<usize, Box<dyn Error>> {
        let mut monkeys = Vec::with_capacity(10);

        for monkey_spec in input.split("\n\n") {
            let mut lines = monkey_spec.lines();
            lines
                .next()
                .expect("there should be a header, even if we ignore it");

            let items = lines.next().unwrap()[STARTING_ITEMS_PREFIX.len()..]
                .split(",")
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect_vec();

            let operation_parts = lines.next().unwrap()[OPERATIONS_PREFIX.len()..]
                .split_whitespace()
                .collect_vec();
            let op = match operation_parts[0].chars().nth(0).unwrap() {
                '+' => Op::Add(operation_parts[1].parse().unwrap()),
                '*' => match operation_parts[1] {
                    "old" => Op::Square,
                    _ => Op::Mul(operation_parts[1].parse().unwrap()),
                },
                _ => panic!("Unkown op {operation_parts:?}"),
            };
            let test = Test::Divisible(lines.next().unwrap()[TEST_PREFIX.len()..].parse().unwrap());
            let positive_target: usize = lines.next().unwrap()[POSITIVE_ACTION_PREFIX.len()..]
                .parse()
                .unwrap();
            let negative_target: usize = lines.next().unwrap()[NEGATIVE_ACTION_PREFIX.len()..]
                .parse()
                .unwrap();

            let monkey = Monkey {
                item_worry_levels: items,
                op,
                test,
                positive_target,
                negative_target,
                inspection_count: 0,
            };

            monkeys.push(RefCell::new(monkey));
        }

        println!("Monkeys:\n{monkeys:?}");

        let lcm = monkeys
            .iter()
            .map(|monkey| monkey.borrow().test)
            .map(|test| match test {
                Test::Divisible(d) => d,
            })
            .product::<usize>();

        for _ in 0..number_of_rounds {
            for monkey_cell in &monkeys {
                let mut monkey = monkey_cell.borrow_mut();
                let old_worry_levels = monkey.item_worry_levels.drain(0..).collect_vec();
                for item in old_worry_levels {
                    monkey.inspection_count += 1;
                    let new = (monkey.op.execute(item) / worry_reduction) % lcm;
                    let target_monkey = match monkey.test.apply(new) {
                        true => &monkeys[monkey.positive_target],
                        false => &monkeys[monkey.negative_target],
                    };
                    target_monkey.borrow_mut().item_worry_levels.push(new);
                }
            }
        }

        let sorted_counts = monkeys
            .iter()
            .map(|monkey_cell| monkey_cell.borrow().inspection_count)
            .sorted()
            .rev()
            .collect_vec();
        let monkey_business_level = sorted_counts[0] * sorted_counts[1];
        Ok(monkey_business_level)
    }
}
