#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error, ops::RangeInclusive, str::FromStr};

    use itertools::Itertools;
    use num_bigint::{BigInt, ToBigInt};
    use regex::Regex;

    use crate::common::tests::{get_input, get_sample_input};

    type Ordinal = i32;
    type Coord = (Ordinal, Ordinal);

    #[test]
    fn day15_part1_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(15)?;
        let row = 10;

        let total = find_positions_that_cannot_have_beacons(&input, row)?;

        assert!(total == 26);

        Ok(())
    }

    #[test]
    fn day15_part1_full() -> Result<(), Box<dyn Error>> {
        let input = get_input(15)?;
        let row = 2_000_000;

        let total = find_positions_that_cannot_have_beacons(&input, row)?;

        println!("number of positions that cannot contain a beacon: {total}");

        Ok(())
    }

    fn find_positions_that_cannot_have_beacons(input: &str, row: i32) -> Result<i32, Box<dyn Error>> {
        let pairs = parse_input(&input)?;

        let (disjoint_ranges, beacons_on_row) = find_regions_covered_by_sensors(&pairs, row);

        let mut total = 0;
        let mut covered_beacons_count = 0;
        for r in disjoint_ranges {
            let size = r.end() - r.start() + 1;
            total += size;
            for beacon in &beacons_on_row {
                if r.contains(&beacon.0) {
                    covered_beacons_count += 1;
                }
            }
        }

        Ok(total - covered_beacons_count)
    }

    #[test]
    fn day15_part2_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(15)?;
        let search_space = 20;

        let pairs = parse_input(&input)?;
        let possible_locations = search_for_open_beacon_positions(&pairs, search_space);

        assert!(possible_locations.len() == 1);

        let result = possible_locations.first().unwrap();
        let (x, y) = result.0;
        let frequency = result.1.clone();
        assert!(x == 14);
        assert!(y == 11);
        assert!(Ok(frequency) == BigInt::from_str("56000011"));

        Ok(())
    }

    #[test]
    fn day15_part2_full() -> Result<(), Box<dyn Error>> {
        let input = get_input(15)?;
        let search_space = 4_000_000;

        let pairs = parse_input(&input)?;
        let possible_locations = search_for_open_beacon_positions(&pairs, search_space);

        for location in possible_locations {
            let (x, y) = location.0;
            let tuning_frequency = location.1;
            println!("room for beacon at ({x}, {y}), tuning frequency: {tuning_frequency}");
        }

        Ok(())
    }

    fn search_for_open_beacon_positions(
        pairs: &Vec<(Coord, Coord, Ordinal)>,
        search_space: i32,
    ) -> Vec<(Coord, BigInt)> {
        let mut possible_locations = Vec::new();

        for row in 0..search_space {
            let (disjoint_ranges, _) = find_regions_covered_by_sensors(&pairs, row);
            if disjoint_ranges.is_empty() {
                continue;
            }

            let first_start = *(disjoint_ranges.first().unwrap().start());
            if first_start > 0 {
                //there's a gap here!
                for x in 0..first_start {
                    possible_locations.push(((x, row), tuning_frequency(x, row)));
                }
            }

            for (one, two) in disjoint_ranges.iter().tuple_windows() {
                let start_one = *one.start();
                let end_one = *one.end();
                let start_two = *two.start();
                let end_two = *two.end();

                if start_one > search_space || end_two < 0 {
                    continue;
                }

                let gap_start = end_one + 1;
                let gap_end = start_two - 1;

                if gap_end < 0 || gap_start > search_space {
                    continue;
                }

                let clamped_start = gap_start.max(0);
                let clamped_end = gap_end.min(search_space);

                for gap_x in clamped_start..=clamped_end {
                    possible_locations.push(((gap_x, row), tuning_frequency(gap_x, row)));
                }
            }

            let last_end = *(disjoint_ranges.last().unwrap().end());
            if last_end < search_space {
                for x in last_end + 1..=search_space {
                    possible_locations.push(((x, row), tuning_frequency(x, row)));
                }
            }
        }

        possible_locations
    }

    fn tuning_frequency(x: i32, y: i32) -> BigInt {
        let gap_x: BigInt = x.to_bigint().unwrap();
        let row = y.to_bigint().unwrap();
        let multip = 4_000_000.to_bigint().unwrap();
        let possible_tuning_frequency = gap_x * multip + row;

        possible_tuning_frequency
    }

    fn parse_input(input: &str) -> Result<Vec<(Coord, Coord, Ordinal)>, Box<dyn Error>> {
        let regex = Regex::new("Sensor at x=(?P<sx>-?\\d+), y=(?P<sy>-?\\d+): closest beacon is at x=(?P<bx>-?\\d+), y=(?P<by>-?\\d+)")?;

        let mut pairs: Vec<(Coord, Coord, Ordinal)> = Vec::new();

        for line in input.lines() {
            let captures = regex.captures(line).unwrap();
            let sx: Ordinal = captures.name("sx").unwrap().as_str().parse()?;
            let sy: Ordinal = captures.name("sy").unwrap().as_str().parse()?;
            let bx: Ordinal = captures.name("bx").unwrap().as_str().parse()?;
            let by: Ordinal = captures.name("by").unwrap().as_str().parse()?;

            let manhattan_distance = (bx - sx).abs() + (by - sy).abs();

            pairs.push(((sx, sy), (bx, by), manhattan_distance));
        }

        Ok(pairs)
    }

    fn find_regions_covered_by_sensors(
        pairs: &Vec<(Coord, Coord, Ordinal)>,
        row: i32,
    ) -> (Vec<RangeInclusive<i32>>, HashSet<Coord>) {
        let mut relevant_sensors: Vec<Coord> = Vec::new();
        let mut covered_ranges: Vec<RangeInclusive<i32>> = Vec::new();
        let mut beacons_on_row = HashSet::new();

        for (sensor, beacon, manhattan_distance) in pairs {
            let (sx, sy) = *sensor;

            if beacon.1 == row {
                beacons_on_row.insert(*beacon);
            }

            let dist_to_row = (row - sy).abs();

            if dist_to_row <= *manhattan_distance {
                relevant_sensors.push(*sensor);

                let spread = manhattan_distance - dist_to_row;
                let covered_range = (sx - spread)..=(sx + spread);
                covered_ranges.push(covered_range);
            }
        }

        covered_ranges.sort_by_key(|range| range.start().clone());

        let mut disjoint_ranges: Vec<RangeInclusive<i32>> = Vec::new();

        for range in covered_ranges {
            let current_range = disjoint_ranges.last_mut();

            if current_range == None {
                disjoint_ranges.push(range);
                continue;
            }
            let last = current_range.unwrap();
            if *range.start() <= *last.end() + 1 {
                //intersecting with or touching current range !
                let start = last.start().clone();
                let end = last.end().max(range.end()).clone();
                *last = start..=end;
            } else {
                //no intersection: start of a new disjoint range
                disjoint_ranges.push(range.clone());
            }
        }

        (disjoint_ranges, beacons_on_row)
    }
}
