#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;
    use std::collections::HashSet;

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
}
