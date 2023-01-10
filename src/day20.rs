#[cfg(test)]
mod tests {
    use std::error::Error;

    use itertools::Itertools;
    use num::Integer;

    use crate::common::tests::{get_input, get_sample_input};

    #[test]
    fn day20_part1_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(20)?;
        let answer = decrypt(&input, 1, 1)?;

        assert!(answer == 3);

        Ok(())
    }

    #[test]
    fn day20_part2_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(20)?;
        let answer = decrypt(&input, 811589153, 10)?;

        assert!(answer == 1623178306);

        Ok(())
    }

    #[test]
    fn day20_part1() -> Result<(), Box<dyn Error>> {
        let input = get_input(20)?;
        let answer = decrypt(&input, 1, 1)?;
        println!("part1: {answer}");

        Ok(())
    }
    #[test]
    fn day20_part2() -> Result<(), Box<dyn Error>> {
        let decryption_key = 811589153;
        let iteration_count = 10;
        let input = get_input(20)?;
        let answer = decrypt(&input, decryption_key, iteration_count)?;
        println!("part2: {answer}");

        Ok(())
    }

    fn decrypt(
        input: &str,
        decryption_key: i64,
        iteration_count: usize,
    ) -> Result<i64, Box<dyn Error>> {
        let mut numbers = input
            .lines()
            .enumerate()
            .map(|(position, line)| {
                (
                    line.parse::<i64>().unwrap() * decryption_key,
                    position,
                    false,
                )
            })
            .collect_vec();
        let count = numbers.len();

        for _ in 0..iteration_count {
            numbers.iter_mut().for_each(|(_, _, p)| *p = false);

            for c in 0..count {
                let i = numbers.iter().position(|(_, pos, _)| pos == &c).unwrap();
                let (number, position, processed) = numbers[i];
                if !processed {
                    let new_index = (i as i64 + number).mod_floor(&(count as i64 - 1));
                    let mut new_index = new_index as usize;
                    // println!("processing {number}: {i} -> {new_index}");
                    if new_index == i {
                        // nothing to do, just mark as processed and go on
                        numbers.get_mut(i).unwrap().2 = true;
                    } else {
                        numbers.remove(i);
                        if new_index == 0 {
                            new_index = count - 1;
                        }
                        numbers.insert(new_index, (number, position, true));
                    }
                }
            }
        }

        let count = count as usize;
        let zero_index = numbers
            .iter()
            .position(|(number, _, _)| number == &0)
            .unwrap();

        let result = numbers[(zero_index + 1000) % count].0
            + numbers[(zero_index + 2000) % count].0
            + numbers[(zero_index + 3000) % count].0;

        Ok(result)
    }
}
