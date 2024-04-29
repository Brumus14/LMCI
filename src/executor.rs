use std::io::stdin;

pub fn execute(program: Vec<u16>) {
    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: i16 = 0;
    let mut program_counter: u8 = 0;
    let mut current_instruction: u16 = 0;
    let mut halt = false;

    // Load program into memory
    for d in 0..program.len() {
        memory[d] = program[d];
    }

    while !halt {
        current_instruction = memory[program_counter as usize];
        program_counter += 1;

        let opcode: u8 = (current_instruction / 100) as u8;
        let operand: u8 = (current_instruction % 100) as u8;

        match opcode {
            1 => accumulator = accumulator.wrapping_add(memory[operand as usize] as i16),
            2 => accumulator = accumulator.wrapping_sub(memory[operand as usize] as i16),
            3 => memory[operand as usize] = accumulator as u16,
            5 => accumulator = memory[operand as usize] as i16,
            6 => program_counter = operand,
            7 => {
                if accumulator == 0 {
                    program_counter = operand
                }
            }
            8 => {
                if accumulator >= 0 {
                    program_counter = operand
                }
            }
            9 => match operand {
                1 => {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    accumulator = input.trim().parse::<u16>().unwrap() as i16;
                }
                2 => {
                    println!("{}", accumulator);
                }
                _ => (),
            },
            0 => halt = true,
            _ => (),
        }
    }
}
