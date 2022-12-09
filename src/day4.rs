#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;
    use std::ops::RangeInclusive;

    use itertools::Itertools;

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
}
