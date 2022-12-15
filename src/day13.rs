#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, error::Error, iter::Peekable};

    use itertools::Itertools;

    use crate::common::tests::{get_input, get_sample_input};

    #[test]
    fn day13_part1_sample() -> Result<(), Box<dyn Error>> {
        let indices_sum = day13_part1(get_sample_input(13)?)?;
        assert!(indices_sum == 13);
        Ok(())
    }
    #[test]
    fn day13_part1_full() -> Result<(), Box<dyn Error>> {
        let indices_sum = day13_part1(get_input(13)?)?;
        println!("Sum: {indices_sum}");
        Ok(())
    }

    fn day13_part1(input: String) -> Result<usize, Box<dyn Error>> {
        let mut sum = 0;
        for (pair_index, part) in input.split("\n\n").enumerate() {
            let pair_index = pair_index + 1;
            let mut lines = part.lines();
            let mut left = lines.next().unwrap().chars().peekable();
            let mut right = lines.next().unwrap().chars().peekable();

            assert!(left.next().unwrap() == '[');
            assert!(right.next().unwrap() == '[');

            match compare_lists(&mut left, &mut right) {
                Ordering::Less => {
                    sum += pair_index;
                }
                Ordering::Equal => panic!(),
                Ordering::Greater => {
                    // OK, do nothing
                }
            }
        }

        Ok(sum)
    }

    #[test]
    fn day13_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(13)?;
        let input = input + "\n[[2]]\n[[6]]\n";
        let packets = input
            .lines()
            .filter(|line| !line.is_empty())
            .sorted_by(|left, right| compare_strings(left, right))
            .collect_vec();

        let pos_2 = packets.iter().position(|s| s == &"[[2]]").unwrap() + 1;
        let pos_6 = packets.iter().position(|s| s == &"[[6]]").unwrap() + 1;

        let product = pos_2 * pos_6;
        println!("Product: {product}");

        Ok(())
    }

    #[test]
    fn simple() {
        assert!(compare_strings("[8]", "[9]") == Ordering::Less);
        assert!(compare_strings("[9]", "[9]") == Ordering::Equal);
        assert!(compare_strings("[9]", "[8]") == Ordering::Greater);
    }

    #[test]
    fn nested_single() {
        assert!(compare_strings("[[8]]", "[[9]]") == Ordering::Less);
        assert!(compare_strings("[[9]]", "[[9]]") == Ordering::Equal);
        assert!(compare_strings("[[9]]", "[[8]]") == Ordering::Greater);
    }

    #[test]
    fn list() {
        assert!(compare_strings("[8,9]", "[9,9]") == Ordering::Less);
        assert!(compare_strings("[9,9]", "[9,9]") == Ordering::Equal);
        assert!(compare_strings("[9,9]", "[8,9]") == Ordering::Greater);
    }

    #[test]
    fn nested_lists() {
        assert!(compare_strings("[4,[8,9]]", "[4,[9,9]]") == Ordering::Less);
        assert!(compare_strings("[4,[9,9]]", "[4,[9,9]]") == Ordering::Equal);
        assert!(compare_strings("[4,[9,9]]", "[4,[8,9]]") == Ordering::Greater);
    }

    #[test]
    fn mixed_levels() {
        assert!(compare_strings("[4,8]", "[[5],8]") == Ordering::Less);
        assert!(compare_strings("[4,8]", "[[4],8]") == Ordering::Equal);
        assert!(compare_strings("[4,8]", "[[3],8]") == Ordering::Greater);
    }

    fn compare_strings(left: &str, right: &str) -> Ordering {
        let mut left = left.chars().peekable();
        let mut right = right.chars().peekable();

        assert!(left.next().unwrap() == '[');
        assert!(right.next().unwrap() == '[');

        compare_lists(&mut left, &mut right)
    }

    fn compare_lists<I: Iterator<Item = char>>(
        left: &mut Peekable<I>,
        right: &mut Peekable<I>,
    ) -> Ordering {
        loop {
            let left_char = left.next().unwrap();
            let right_char = right.next().unwrap();

            let current_position_cmp = match (left_char, right_char) {
                ('[', '[') => compare_lists(left, right),
                (']', ']') => {
                    //both end at the same time
                    return Ordering::Equal;
                }
                (']', _) => Ordering::Less, //the left ends, but the right continues
                (_, ']') => Ordering::Greater, //the left continues, but the right ends
                (l, r) if l.is_ascii_digit() && r.is_ascii_digit() => {
                    compare_scalars(parse_scalar(l, left), parse_scalar(r, right))
                }
                (l, '[') if l.is_ascii_digit() => {
                    compare_scalar_with_list(parse_scalar(l, left), right)
                }
                ('[', r) if r.is_ascii_digit() => {
                    compare_scalar_with_list(parse_scalar(r, right), left).reverse()
                }
                _ => panic!("unrecognized chars! left: {left_char} ; right: {right_char}"),
            };

            //proceed to the next position to compare
            if left.peek() == Some(&',') {
                left.next();
            }
            if right.peek() == Some(&',') {
                right.next();
            }

            if current_position_cmp != Ordering::Equal {
                break current_position_cmp;
            }
        }
    }

    fn parse_scalar<I: Iterator<Item = char>>(
        first_char: char,
        peekable: &mut Peekable<I>,
    ) -> usize {
        let mut string = String::new();
        string.push(first_char);

        if let Some(c) = peekable.peek() {
            if c.is_ascii_digit() {
                string.push(peekable.next().unwrap());
            }
        }

        string.parse().unwrap()
    }

    fn compare_scalars(left: usize, right: usize) -> Ordering {
        left.cmp(&right)
    }

    fn compare_scalar_with_list<I: Iterator<Item = char>>(
        scalar: usize,
        list: &mut Peekable<I>,
    ) -> Ordering {
        match list.next().unwrap() {
            '[' => compare_scalar_with_list(scalar, list),
            ']' => return Ordering::Greater, //empty list
            c if c.is_ascii_digit() => {
                let first_scalar_in_list = parse_scalar(c, list);
                match scalar.cmp(&first_scalar_in_list) {
                    Ordering::Equal => {
                        //check if list has more items
                        if list.next().unwrap() != ']' {
                            return Ordering::Less;
                        } else {
                            return Ordering::Equal;
                        }
                    }
                    r => return r,
                }
            }
            _ => panic!("The list doesn't start with a digit or another list!"),
        }
    }
}
