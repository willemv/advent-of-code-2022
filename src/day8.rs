#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;
    use std::collections::HashSet;

    use transpose::transpose;

    #[test]
    fn day8() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(8)?;

        let mut width = None;
        let mut height = 0;
        let mut matrix: Vec<usize> = vec![];
        for line in input.lines() {
            height += 1;
            if width == None {
                width = Some(line.len());
            }
            for char in line.chars() {
                matrix.push(char.to_string().parse().unwrap());
            }
        }

        let width = width.unwrap();
        fn collect_visible_trees(matrix: &[usize], width: usize) -> HashSet<(usize, usize)> {
            let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
            let mut max_tree_height = None;
            for (i, tree_height) in matrix.iter().enumerate() {
                let (row, column) = (i / width, i % width);
                if column == 0 {
                    max_tree_height = None;
                }

                let coord = (row, column);
                if let Some(height) = max_tree_height {
                    if tree_height > height {
                        visible_trees.insert(coord);
                        max_tree_height = Some(tree_height);
                    }
                } else {
                    visible_trees.insert(coord);
                    max_tree_height = Some(tree_height);
                }
            }

            max_tree_height = None;

            for (j, tree_height) in matrix.iter().rev().enumerate() {
                let i = matrix.len() - 1 - j;
                let (row, column) = (i / width, i % width);

                if column == width - 1 {
                    max_tree_height = None;
                }

                let coord = (row, column);
                if let Some(current_max_height) = max_tree_height {
                    if tree_height > current_max_height {
                        visible_trees.insert(coord);
                        max_tree_height = Some(tree_height);
                    }
                } else {
                    visible_trees.insert(coord);
                    max_tree_height = Some(tree_height);
                }
            }
            visible_trees
        }

        let mut visible_trees = collect_visible_trees(&matrix, width);
        let mut transposed = vec![0; matrix.len()];

        transpose(&matrix, &mut transposed, width, height);
        let transposed_visible_trees = collect_visible_trees(&transposed, height);
        visible_trees.extend(transposed_visible_trees.into_iter().map(|(x, y)| (y, x)));

        println!(
            "Number of trees visible from the outside: {}",
            visible_trees.len()
        );

        fn scenic_score(
            x: usize,
            y: usize,
            forest: &[usize],
            forest_width: usize,
            forest_height: usize,
        ) -> (usize, usize, usize, usize) {
            let tree_index = (y * forest_width) + x;
            let tree_height = forest[tree_index];

            let mut score_west = 0;
            {
                let mut i = tree_index;
                while i > y * forest_width {
                    score_west += 1;
                    i -= 1; //cannot underflow because of condition above;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }

            let mut score_north = 0;
            {
                let mut i = tree_index;
                while i > forest_width {
                    score_north += 1;
                    i -= forest_width; //up a row (cannot underflow because of while condition)
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }
            let mut score_east = 0;
            {
                let mut i = tree_index;
                let end_of_row = ((y + 1) * forest_width) - 1;
                while i < end_of_row {
                    score_east += 1;
                    i += 1;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }
            let mut score_south = 0;
            {
                let mut i = tree_index;
                let end_of_col = (forest_width * (forest_height - 1)) + x;
                while i < end_of_col {
                    score_south += 1;
                    i += forest_width;
                    if forest[i] >= tree_height {
                        break;
                    }
                }
            }

            (score_north, score_east, score_south, score_west)
        }

        let mut max_score = 0;
        for y in 0..height {
            for x in 0..width {
                let scores = scenic_score(x, y, &matrix, width, height);
                let score = scores.0 * scores.1 * scores.2 * scores.3;
                if score >= max_score {
                    println!("new highscore for ({x}, {y}): {score} ({scores:?})");
                    max_score = score;
                }
            }
        }

        Ok(())
    }
}
