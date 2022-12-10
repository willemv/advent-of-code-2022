#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::common::tests::get_input;

    enum Instruction {
        Noop,
        AddX(i32),
    }

    impl Instruction {
        fn cycles(&self) -> usize {
            match *self {
                Instruction::Noop => 1,
                Instruction::AddX(_) => 2,
            }
        }

        fn execute(&self, register: &mut i32) {
            match *self {
                Instruction::Noop => { /* do nothing */ }
                Instruction::AddX(inc) => *register += inc,
            }
        }
    }

    #[test]
    fn day10() -> Result<(), Box<dyn Error>> {
        let input = get_input(10)?;

        let mut x_register = 1i32;
        let mut completed_cycles = 0usize;

        let mut signal_strength_sum = 0i32;
        for line in input.lines() {
            let instruction = if line.starts_with("noop") {
                Instruction::Noop
            } else if line.starts_with("addx ") {
                let to_add: i32 = (&line[5..]).parse()?;
                Instruction::AddX(to_add)
            } else {
                panic!("Unrecognized operand");
            };

            let cycles_for_current_op = instruction.cycles();

            //will the 20th, 60th, 100th, ... cycle be between the start of and of the current operand execution?
            let adjusted_current_cycle = completed_cycles % 40;
            if adjusted_current_cycle < 20 && adjusted_current_cycle + cycles_for_current_op >= 20 {
                //hit! the current X value is the one we need
                let relevant_cycle = completed_cycles - adjusted_current_cycle + 20;
                signal_strength_sum += x_register * (relevant_cycle as i32);
            }

            //we know the value of X for the coming #cycles_for_current_op cycles
            for i in 0..cycles_for_current_op {
                let pixel_being_drawn = (completed_cycles + i) % 40;
                let pixel_being_drawn = pixel_being_drawn as i32;

                if pixel_being_drawn >= x_register - 1 && pixel_being_drawn <= x_register + 1 {
                    print!("█");
                } else {
                    print!(".");
                }

                if pixel_being_drawn == 39 {
                    println!();
                }
            }

            instruction.execute(&mut x_register);
            completed_cycles += cycles_for_current_op;
        }

        println!("Sum of signal strengths: {signal_strength_sum}");
        Ok(())
    }
}
