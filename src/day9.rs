#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;
    use std::collections::HashSet;
    use std::{thread, time};

    #[test]
    fn day9_part1() -> Result<(), Box<dyn std::error::Error>> {
        day9_impl(2, false)
    }

    #[test]
    fn day9_debug() -> Result<(), Box<dyn std::error::Error>> {
        day9_impl(10, true)
    }

    #[test]
    fn day9_part2() -> Result<(), Box<dyn std::error::Error>> {
        day9_impl(10, false)
    }

    fn day9_impl(rope_size: usize, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_size];

        fn touching(head: &(i32, i32), tail: &(i32, i32)) -> bool {
            (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
        }

        fn diag(target: &(i32, i32), current: &(i32, i32)) -> (i32, i32) {
            (
                (target.0 - current.0).signum(),
                (target.1 - current.1).signum(),
            )
        }

        fn print_rope(rope: &[(i32, i32)]) {
            let mut set = HashSet::new();
            for c in rope {
                set.insert(c.clone());
            }
            for y in (-25..25).rev() {
                for x in -15..15 {
                    if rope[0] == (x, y) {
                        print!("H")
                    } else if set.contains(&(x, y)) {
                        print!("X")
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }

        let mut tail_visited = HashSet::new();
        tail_visited.insert(rope.last().unwrap().clone());

        let input = get_input(9)?;
        // let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        for line in input.lines() {
            let direction = line.chars().nth(0).unwrap();
            let count: usize = (&line[2..]).parse()?;

            let dir = match direction {
                'D' => (0, -1),
                'U' => (0, 1),
                'R' => (1, 0),
                'L' => (-1, 0),
                _ => panic!("Unsupported dir char {direction}"),
            };

            for _ in 0..count {
                let head = rope.first_mut().unwrap();
                head.0 += dir.0;
                head.1 += dir.1;

                for i in 0..rope.len() - 1 {
                    let (car, cdr) = rope.split_at_mut(i + 1);
                    let front = car.last().unwrap();
                    let back = cdr.first_mut().unwrap();

                    if !touching(front, back) {
                        let d = diag(&front, &back);
                        back.0 += d.0;
                        back.1 += d.1;
                    }
                }

                if debug {
                    print_rope(&rope);
                    println!();
                    let ten_millis = time::Duration::from_millis(300);
                    thread::sleep(ten_millis);
                }

                tail_visited.insert(rope.last().unwrap().clone());
            }
        }

        println!(
            "Number of places visited by the tail: {}",
            tail_visited.len()
        );

        Ok(())
    }
}
