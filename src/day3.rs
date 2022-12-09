#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;
    use std::collections::HashSet;

    use itertools::Itertools;

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
}
