#[cfg(test)]
mod tests {
    use crate::common::tests::{get_input, get_sample_input};
    use std::collections::HashSet;
    use std::error::Error;

    #[derive(Copy, Clone, Debug)]
    enum Block {
        Line,
        Cross,
        MirroredEl,
        I,
        Square,
    }

    type Ord = i64;
    type Coord = (Ord, Ord);

    fn points(block: &Block) -> Vec<Coord> {
        match *block {
            Block::Line => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Block::Cross => vec![(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Block::MirroredEl => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Block::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Block::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn next_position_free(
        current: &Coord,
        direction: &Coord,
        occupied_positions: &HashSet<Coord>,
        block: &Block,
        field_width: Ord,
    ) -> bool {
        let position = current;
        let jet_dir = direction;
        let (pos_x, pos_y) = position;

        for (relative_x, relative_y) in points(block) {
            let absolute_x = relative_x + pos_x;
            let absolute_y = relative_y + pos_y;
            let future_x = absolute_x + jet_dir.0;
            let future_y = absolute_y + jet_dir.1;
            if occupied_positions.contains(&(future_x, future_y))
                || future_x >= field_width
                || future_x < 0
                || future_y < 0
            {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn day17() -> Result<(), Box<dyn Error>> {

        // in sample input: 
        // there's a pattern of 53 lines (for every 35 blocks) that keeps repeating after a prelude of 36 lines (from the first 20 blocks)
        // checking the output, the last remaining 30 blocks go 42 lines high
        
        // 1_000_000_000_000 - 20 = 999999999980 # minus the number of blocks in the prelude
        // 999999999980 / 35 = 28571428570       # number of times the pattern repeats
        // 999999999980 % 35 = 30                # remaining lines
        // 28571428570 * 53 = 1514285714210

        // total: 36 + (28571428570 * 53) + 42 = 1514285714288 # which matches the example of day 17
        // let input = get_sample_input(17)?;


        // in real input:
        // there is a prelude of 3460 lines for 2185 blocks
        // there is a pattern of 2781 lines for every 1735 blocks
        // there is a final block of 2293 lines for the remaining 1425 blocks
        // total height after 1_000_000_000_000 blocks = 3460 + (576368874 * 2781) + 2293 = 1602881844347 lines

        // repeating pattern of 784 - 437 + 1 = 348 blocks ; 6240 - 3459 = 2781 lines
        // repeating pattern of 347 * 5 = 1735 blocks ; 6240 - 3459 = 2781 lines
        // block count of repeating pattern: 3920 - 2186 + 1 = 1735
        // 1_000_000_000_000 - 2185 = 999999997815  # minus the number of blocks in the prelude
        // 999999997815 / 1735 = 576368874          # number of times the pattern repeats
        // 999999997815 % 1735 = 1425               # remaining lines
        
        // all extracted from this output, as generated below
        //(line : log)
        //
        //    1 : Placing line block at 2 (j%len(): 3)  - false - max_height before -1
        // ...
        // 2186 : Placing line block at 2 (j%len(): 2712)  - true - max_height before 3459    ----> BEGIN OF REPEATED PATTERN
        // 2187 : Placing block Cross, max_height before: 3460
        // ...
        // 3609: Placing block I, max_height before: 5746
        // 3610: Placing block Square, max_height before: 5750
        // 3611: Placing line block at 3 (j%len(): 846)  - true - max_height before 5752
        // 3612: Placing block Cross, max_height before: 5753
        // 3613: Placing block MirroredEl, max_height before: 5754
        // ...
        // 3920: Placing block Square, max_height before: 6240
        // 3921: Placing line block at 2 (j%len(): 2712)  - true - max_height before 6240    ----> BEGIN OF REPEATED PATTERN AGAIN
        // 3922: Placing block Cross, max_height before: 6241
        
        let input = get_input(17)?;

        let input = input.trim();
        let num_of_jets = input.len();

        println!("Input length: {}", input.len());

        let blocks = vec![
            Block::Line,
            Block::Cross,
            Block::MirroredEl,
            Block::I,
            Block::Square,
        ];

        let num_of_blocks = blocks.len();

        println!("i64::MAX: {}", i64::MAX);
        // let block_count = 2022;
        let block_count = 1_000_000_000_000;
        let mut blocks = blocks.iter().cycle().take(block_count);
        let mut jets = input.chars().cycle().enumerate();

        let mut occupied_positions: HashSet<Coord> = HashSet::new();
        let mut highest_rock = -1;
        const FIELD_WIDTH: usize = 7;
        let mut skyline = [-1; FIELD_WIDTH];
        let field_width = FIELD_WIDTH as Ord;

        let mut interesting = HashSet::new();

        for (i, block) in blocks.enumerate() {
            // insert block
            let mut position = (2, highest_rock + 4);

            loop {
                //push block if possible
                let (j, jet) = jets.next().unwrap();

                let jet_dir = match jet {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    _ => panic!("Unsupported input! {jet}"),
                };

                let next_free = next_position_free(
                    &position,
                    &jet_dir,
                    &occupied_positions,
                    block,
                    field_width,
                );

                if next_free {
                    position = (position.0 + jet_dir.0, position.1 + jet_dir.1);
                } else {
                }

                let down_free = next_position_free(
                    &position,
                    &(0, -1),
                    &occupied_positions,
                    block,
                    field_width,
                );

                if down_free {
                    position = (position.0, position.1 - 1);
                } else {
                    //place block

                    if i % num_of_blocks == 0 {
                        //first line block, check if we positioned it at that x before, AT THE SAME POSITION IN THE JET INPUT STREAM
                        let (pos_x, _) = position;
                        let pos_in_jets = j % num_of_jets;

                        let int=  interesting.contains(&(pos_x, pos_in_jets)) ;
                        interesting.insert((pos_x, pos_in_jets));

                        println!("Placing line block at {pos_x} (j%len(): {})  - {int} - max_height before {highest_rock}", j % num_of_jets);

                    } else {
                        println!("Placing block {block:?}, max_height before: {highest_rock}");
                    }

                    for (point_x, point_y) in points(block) {
                        let (relative_x, relative_y) = position;
                        let (absolute_x, absolute_y) = (relative_x + point_x, relative_y + point_y);
                        let x = absolute_x as usize;
                        skyline[x] = skyline[x].max(absolute_y);

                        occupied_positions.insert((relative_x + point_x, relative_y + point_y));
                        highest_rock = highest_rock.max(absolute_y);
                    }

                    break;
                }
            }
        }

        println!("highest rock: {}", highest_rock + 1);

        Ok(())
    }
}
