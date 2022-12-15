#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        error::Error,
        fmt::{Display, Formatter},
    };

    use itertools::Itertools;

    use crate::common::tests::{get_input, get_sample_input};

    type Coord = (i32, i32);

    #[derive(Debug, Clone, Copy)]
    enum OccupiedWith {
        Wall,
        Sand,
    }

    #[derive(Debug, Clone)]
    struct Field {
        occupied_cells: HashMap<Coord, OccupiedWith>,
        top_left: Coord,
        bottom_right: Coord,
    }

    impl Field {
        fn from_input(input: &str) -> Field {
            let mut grid = HashMap::new();
            let mut top_left = (i32::MAX, 0);
            let mut bottom_right = (i32::MIN, i32::MIN);

            fn update_extremes(current: &Coord, top_left: &mut Coord, bottom_right: &mut Coord) {
                if current.0 < top_left.0 {
                    top_left.0 = current.0;
                }
                if current.1 < top_left.1 {
                    top_left.1 = current.1;
                }

                if current.0 > bottom_right.0 {
                    bottom_right.0 = current.0;
                }
                if current.1 > bottom_right.1 {
                    bottom_right.1 = current.1;
                }
            }

            fn parse_coord(input: &str) -> Coord {
                let mut split = input.split(',');
                (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            }

            for line in input.lines() {
                let vertices = line.split(" -> ").map(parse_coord).collect_vec();

                for (a, b) in vertices.into_iter().tuple_windows() {
                    let direction = if a.0 == b.0 {
                        (0, (b.1 - a.1).signum())
                    } else if a.1 == b.1 {
                        ((b.0 - a.0).signum(), 0)
                    } else {
                        panic!();
                    };

                    let mut current = a;
                    while current != b {
                        grid.insert(current, OccupiedWith::Wall);
                        update_extremes(&current, &mut top_left, &mut bottom_right);

                        current.0 += direction.0;
                        current.1 += direction.1;
                    }

                    grid.insert(b, OccupiedWith::Wall);
                    update_extremes(&b, &mut top_left, &mut bottom_right);
                }
            }

            Field {
                occupied_cells: grid,
                top_left,
                bottom_right,
            }
        }
    }

    impl Display for &Field {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            let grid = &self.occupied_cells;
            let bottom_right = &self.bottom_right;
            let top_left = &self.top_left;

            let field_width = bottom_right.0 - top_left.0 + 1;
            let field_height = bottom_right.1 - top_left.1 + 1;
            let padding = 40;

            for y in 0..field_height + 5 {
                for x in -padding..field_width + padding {
                    let x = x + top_left.0;
                    let y = y + top_left.1;

                    if x == 500 && y == 0 {
                        write!(formatter, "+")?;
                    } else {
                        match grid.get(&(x, y)) {
                            None => write!(formatter, ".")?,
                            Some(&OccupiedWith::Wall) => write!(formatter, "█")?, //█
                            Some(&OccupiedWith::Sand) => write!(formatter, "0")?,
                        }
                    }
                }
                writeln!(formatter)?;
            }
            Ok(())
        }
    }

    #[test]
    fn day14_part1_sample() -> Result<(), Box<dyn Error>> {
        let count = day14_part1(&get_sample_input(14)?)?;
        assert!(count == 24);
        Ok(())
    }

    #[test]
    fn day14_part1_full() -> Result<(), Box<dyn Error>> {
        let count = day14_part1(&get_input(14)?)?;
        println!("{count} grains of sand were dropped");
        Ok(())
    }

    fn day14_part1(input: &str) -> Result<usize, Box<dyn Error>> {
        let mut field = Field::from_input(&input);

        let mut counter = 0;
        while simulate_grain_part1(&mut field) {
            counter += 1;
        }
        println!("{}", &field);

        Ok(counter)
    }

    #[test]
    fn day14_part2_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(14)?;
        let count = day14_part2(&input)?;
        assert!(count == 93);
        Ok(())
    }

    #[test]
    fn day14_part2_full() -> Result<(), Box<dyn Error>> {
        let input = get_input(14)?;
        let count = day14_part2(&input)?;
        println!("{count} grains of sand were dropped");
        Ok(())
    }

    fn day14_part2(input: &str) -> Result<usize, Box<dyn Error>> {
        let mut field = Field::from_input(&input);

        let mut counter = 0;
        while simulate_grain_part2(&mut field) {
            counter += 1;
        }
        println!("{counter} grains of sand were dropped");
        println!("{}", &field);

        Ok(counter)
    }

    fn simulate_grain_part1(field: &mut Field) -> bool {
        let grid = &mut field.occupied_cells;
        let bottom = field.bottom_right.1;

        let mut grain = (500, 0);
        loop {
            if grain.1 > bottom {
                return false;
            }

            if !grid.contains_key(&(grain.0, grain.1 + 1)) {
                grain.1 += 1; //move down
            } else if !grid.contains_key(&(grain.0 - 1, grain.1 + 1)) {
                grain.0 -= 1;
                grain.1 += 1;
            } else if !grid.contains_key(&(grain.0 + 1, grain.1 + 1)) {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                grid.insert(grain, OccupiedWith::Sand);
                return true;
            }
        }
    }

    fn simulate_grain_part2(field: &mut Field) -> bool {
        let bottom = field.bottom_right.1;

        let grid = &mut field.occupied_cells;
        if grid.contains_key(&(500, 0)) {
            return false;
        }

        let mut grain = (500, 0);
        loop {
            let level_below = grain.1 + 1;
            if grain.1 > bottom {
                grid.insert(grain, OccupiedWith::Sand);
                return true;
            }

            if !grid.contains_key(&(grain.0, level_below)) {
                grain.1 += 1; //move down
            } else if !grid.contains_key(&(grain.0 - 1, level_below)) {
                grain.0 -= 1;
                grain.1 += 1;
            } else if !grid.contains_key(&(grain.0 + 1, level_below)) {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                grid.insert(grain, OccupiedWith::Sand);
                return true;
            }
        }
    }
}
