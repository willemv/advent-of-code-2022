#[cfg(test)]
mod tests {
    use std::{
        collections::{HashSet, VecDeque},
        error::Error,
    };

    use itertools::Itertools;

    use crate::common::tests::{get_input, get_sample_input};

    type Ord = i32;
    type Coord = (Ord, Ord, Ord);

    #[test]
    fn day18() -> Result<(), Box<dyn Error>> {
        // let input = get_sample_input(18)?;
        let input = get_input(18)?;
   
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_z = i32::MAX;
        let mut max_z = i32::MIN;

        let mut occupied_cells = HashSet::new();
        for line in input.lines() {
            let (x, y, z) = line.split(",").tuples().next().unwrap();

            let (x, y, z) = (x.parse::<Ord>()?, y.parse::<Ord>()?, z.parse::<Ord>()?);

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            min_z = min_z.min(z);
            max_z = max_z.max(z);

            occupied_cells.insert((x, y, z));
        }

        println!("number of cells {}", occupied_cells.len());

        let mut empty_cells = HashSet::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let c = (x, y, z);
                    if !occupied_cells.contains(&c) {
                        empty_cells.insert(c);
                    }
                }
            }
        }

        // redeclars as non-mutable
        let empty_cells = empty_cells;
        // println!("empty cells: {empty_cells:?}");

        let directions = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        let total_surface = {
            // let first = occupied_cells.iter().next().unwrap();
            let mut unvisited_cells = HashSet::with_capacity(occupied_cells.len());
            for occupied_cell in occupied_cells.iter() {
                unvisited_cells.insert(occupied_cell);
            }
            let mut visited_cells = HashSet::new();
            let mut frontier = VecDeque::new();

            let mut surface = 0;

            loop {
                let mut current = frontier.pop_front();
                if current.is_none() {
                    if unvisited_cells.is_empty() {
                        break;
                    }
                    // println!("starting a new section");
                    let some = unvisited_cells.iter().next().unwrap();
                    current = Some(unvisited_cells.take(*some).unwrap());
                } else {
                    // println!("continuing section");
                }
                let current = current.unwrap();
                if !visited_cells.insert(current) {
                    continue;
                }
                unvisited_cells.remove(current);
                // println!(
                // "Processing {current:?}, visited cells: {}",
                // visited_cells.len()
                // );

                for dir in directions.iter() {
                    let neighbour = (current.0 + dir.0, current.1 + dir.1, current.2 + dir.2);
                    if visited_cells.contains(&neighbour) {
                        continue;
                    }

                    match occupied_cells.get(&neighbour) {
                        Some(cell) => {
                            frontier.push_back(cell);
                        }
                        None => {
                            surface += 1;
                        }
                    }
                }
            }

            println!("total surface area: {surface}");
            surface
        };

        let inner_surface = {
            let mut unvisited_cells = HashSet::with_capacity(empty_cells.len());
            for empty_cell in empty_cells.iter() {
                unvisited_cells.insert(empty_cell);
            }
            let mut visited_cells = HashSet::new();
            let mut frontier = VecDeque::new();

            let mut total_inner_surface = 0;
            let mut inner_surface = 0;
            let mut is_inner_region = true;

            loop {
                let mut current = frontier.pop_front();
                if current.is_none() {
                    if is_inner_region {
                        total_inner_surface += inner_surface;
                    }

                    if unvisited_cells.is_empty() {
                        break;
                    }
                    // println!("starting a new region");
                    is_inner_region = true;
                    inner_surface = 0;
                    let some = unvisited_cells.iter().next().unwrap();
                    current = Some(unvisited_cells.take(*some).unwrap());
                } else {
                    // println!("continuing section");
                }
                let current = current.unwrap();
                // println!("current: {current:?}");

                //are we at the edge ?
                let (x, y, z) = *current;
                if x <= min_x || x >= max_x || y <= min_y || y >= max_y || z <= min_z || z >= max_z
                {
                    // println!("not an inner region");
                    is_inner_region = false;
                }

                if !visited_cells.insert(current) {
                    continue;
                }
                unvisited_cells.remove(current);

                for dir in directions.iter() {
                    let neighbour = (current.0 + dir.0, current.1 + dir.1, current.2 + dir.2);
                    if visited_cells.contains(&neighbour) {
                        continue;
                    }

                    match empty_cells.get(&neighbour) {
                        Some(cell) => {
                            frontier.push_back(cell);
                        }
                        None => {
                            inner_surface += 1;
                        }
                    }
                }
            }

            // println!("total surface area: {total_inner_surface}");
            total_inner_surface
        };

        println!("total surface area: {total_surface}");
        println!("of which inner surface: {inner_surface}");
        println!("relevant surface area: {}", total_surface - inner_surface);

        Ok(())
    }
}
