#[cfg(test)]
mod tests {

    use crate::common::tests::get_input;

    #[test]
    fn day7() -> Result<(), Box<dyn std::error::Error>> {
        let input = get_input(7)?;

        fn traverse<F>(input: &str, mut finish_dir: F) -> Result<usize, Box<dyn std::error::Error>>
        where
            F: FnMut(usize),
        {
            let mut sizes_stack: Vec<usize> = Vec::new();
            let mut total_size = None;
            for line in input.lines() {
                if line.eq("$ cd ..") {
                    let dir_size = sizes_stack.pop().unwrap();
                    finish_dir(dir_size);
                    *(sizes_stack.last_mut().unwrap()) += dir_size; //add the size of a child dir to the current dir
                } else if line.starts_with("$ cd /") {
                    assert!(sizes_stack.is_empty());
                    sizes_stack.push(0);
                } else if line.starts_with("$ cd ") {
                    sizes_stack.push(0);
                } else if line.starts_with("$ ls") {
                    //ignore
                } else if line.starts_with("dir ") {
                    //ignore, we'll get there later
                } else {
                    // line with a size and filename
                    let space = line.find(' ').unwrap();
                    let file_size: usize = (&line[..space]).parse()?;
                    *(sizes_stack.last_mut().unwrap()) += file_size;
                }
            }

            while let Some(dir_size) = sizes_stack.pop() {
                finish_dir(dir_size);
                if let Some(r) = sizes_stack.last_mut() {
                    *r += dir_size;
                } else {
                    println!("Total size: {dir_size}");
                    total_size = Some(dir_size);
                }
            }
            Ok(total_size.unwrap())
        }

        let total_size = {
            let max_size = 100_000;
            let mut sum = 0;
            let total_size = traverse(&input, |dir_size| {
                if dir_size <= max_size {
                    sum += dir_size;
                }
            })?;

            println!("sum of filtered dirs: {sum}");
            total_size
        };

        {
            let space_to_free_up = 30_000_000 - (70_000_000 - total_size);
            let mut size_of_dir_to_delete = usize::MAX;

            traverse(&input, |dir_size| {
                if dir_size >= space_to_free_up && dir_size < size_of_dir_to_delete {
                    size_of_dir_to_delete = dir_size;
                }
            })?;
            println!("size of dir to delete: {size_of_dir_to_delete}");
        }

        Ok(())
    }
}
