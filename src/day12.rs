#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        error::Error,
    };

    use crate::common::tests::{get_input, get_sample_input};

    const LEFT_CHAR: char = '←';
    const RIGHT_CHAR: char = '→';
    const UP_CHAR: char = '↑';
    const DOWN_CHAR: char = '↓';

    struct Map {
        height_data: Vec<u32>,
        width: usize,
        height: usize,
        came_from: Option<HashMap<Coord, Coord>>,
    }

    type Coord = (usize, usize);
    struct Path {
        path: Vec<Coord>,
    }

    impl Path {
        fn steps(&self) -> usize {
            self.path.len() - 1 //the steps between each coordinate, not the number of coordinates
        }
    }

    impl Map {
        fn parse(input: &str) -> (Coord, Map) {
            let mut heights = vec![];
            let mut width = None;
            let mut start = None;
            let mut end = None;
            for (y, line) in input.lines().enumerate() {
                if width == None {
                    width = Some(line.len());
                }
                for (x, c) in line.chars().enumerate() {
                    match c {
                        'S' => {
                            start = Some((x, y));
                            heights.push(1)
                        }
                        'E' => {
                            end = Some((x, y));
                            heights.push(26)
                        }
                        _ => heights.push(match c {
                            'a'..='z' => c as u32 - 96,
                            _ => panic!(),
                        }),
                    };
                }
            }

            let start = start.unwrap();
            let end = end.unwrap();
            let width = width.unwrap();
            let height = heights.len() / width;

            let mut map = Map {
                height_data: heights,
                width,
                height,
                came_from: None,
            };

            map.prepare_search(end);

            (start, map)
        }

        fn height_at(&self, x: usize, y: usize) -> Option<u32> {
            if x >= self.width {
                return None;
            }

            let index = y * self.width + x;
            if index >= self.height_data.len() {
                None
            } else {
                Some(self.height_data[index])
            }
        }

        fn neighbors(&self, c: Coord) -> Vec<Coord> {
            let (x, y) = c;
            let mut result = Vec::with_capacity(4);

            if x >= self.width {
                return result;
            }
            if y >= self.height {
                return result;
            }

            if y > 0 {
                result.push((x, y - 1));
            }
            if x > 0 {
                result.push((x - 1, y));
            }
            if y < self.height - 1 {
                result.push((x, y + 1));
            }
            if x < self.width - 1 {
                result.push((x + 1, y));
            }
            result
        }

        fn to_index(&self, c: &Coord) -> usize {
            c.1 * self.width + c.0
        }

        fn prepare_search(&mut self, end: Coord) {
            let mut frontier = VecDeque::new();
            frontier.push_front(end);

            let mut visited: HashSet<Coord> = HashSet::new();
            visited.insert(end);
            let mut came_from: HashMap<Coord, Coord> = HashMap::new();

            while !frontier.is_empty() {
                let current = frontier.pop_back().unwrap();
                for neighbor in self.neighbors(current) {
                    let current_height = self.height_at(current.0, current.1).unwrap();
                    let neighbor_height = self.height_at(neighbor.0, neighbor.1).unwrap();
                    let reachable_from_current =
                        current_height < neighbor_height || current_height - neighbor_height <= 1;
                    if !reachable_from_current {
                        continue;
                    }

                    if visited.contains(&neighbor) {
                        continue;
                    }

                    frontier.push_front(neighbor);
                    visited.insert(neighbor);
                    came_from.insert(neighbor, current);
                }
            }

            self.came_from = Some(came_from);
        }

        fn find_path(&self, start: &Coord) -> Option<Path> {
            let came_from = self.came_from.as_ref().unwrap();

            if !came_from.contains_key(&start) {
                return None;
            }

            let mut path: Vec<Coord> = Vec::new();
            let mut current = Some(start);
            while let Some(c) = current {
                path.push(c.clone());
                current = came_from.get(&c);
            }

            Some(Path{path})
        }
    }

    #[test]
    fn day12_part1_sample() -> Result<(), Box<dyn Error>> {
        let path = day12(get_sample_input(12)?)?;
        assert!(path.steps() == 31);
        Ok(())
    }

    #[test]
    fn day12_part1() -> Result<(), Box<dyn Error>> {
        let path = day12(get_input(12)?)?;
        assert!(path.steps() == 520);
        Ok(())
    }

    fn find_shortest_path_from_any_start(map: &Map) -> Option<Path> {
        let mut shortest_path = None;
        for y in 0..map.height {
            for x in 0..map.width {
                if map.height_at(x, y).unwrap() != 1 {
                    continue;
                }

                shortest_path = match (map.find_path(&(x, y)), shortest_path) {
                    (None, shortest) => shortest,
                    (Some(path), None) => Some(path),
                    (Some(path), Some(shortest)) => {
                        if shortest.steps() > path.steps() {
                            Some(path)
                        } else {
                            Some(shortest)
                        }
                    }
                }
            }
        }
        shortest_path
    }

    #[test]
    fn day12_part2_sample() -> Result<(), Box<dyn Error>> {
        let input = get_sample_input(12)?;
        let (_, map) = Map::parse(&input);

        let shortest_path = find_shortest_path_from_any_start(&map);

        if let Some(shortest) = shortest_path {
            visualize_path_on_map(&map, &shortest);
            assert!(shortest.steps() == 29)
        } else {
            panic!("No shortest path found");
        }

        Ok(())
    }

    #[test]
    fn day12_part2() -> Result<(), Box<dyn Error>> {
        let input = get_input(12)?;
        let (_, map) = Map::parse(&input);

        let shortest_path = find_shortest_path_from_any_start(&map);

        if let Some(shortest) = shortest_path {
            visualize_path_on_map(&map, &shortest);
            println!("shortest found: {}", shortest.steps());
        }

        Ok(())
    }

    fn visualize_path_on_map(map: &Map, path: &Path) {
        let mut formatted = vec!['.'; map.height_data.len()];

        for w in path.path.windows(2) {
            let c = w[0];
            let t = w[1];
            let diff = (t.0 as i32 - c.0 as i32, t.1 as i32 - c.1 as i32);

            let ch = match diff {
                (1, 0) => RIGHT_CHAR,
                (-1, 0) => LEFT_CHAR,
                (0, 1) => DOWN_CHAR,
                (0, -1) => UP_CHAR,
                _ => '!',
            };
            formatted[map.to_index(&c)] = ch;
        }
        // formatted[map.to_index(path.first().unwrap())] = 'S';
        formatted[map.to_index(path.path.last().unwrap())] = 'E';

        for (i, c) in formatted.iter().enumerate() {
            if i % map.width == 0 {
                println!();
            }
            print!("{c}");
        }
        println!();
    }

    fn day12(input: String) -> Result<Path, Box<dyn Error>> {
        let (start, map) = Map::parse(&input);

        let path = map.find_path(&start).unwrap();

        visualize_path_on_map(&map, &path);

        Ok(path)
    }
}
