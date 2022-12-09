#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use crate::common::tests::get_input;

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
}
