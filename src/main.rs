use std::{env, fs, process::exit};
enum Instruction {
    Read,      // put the read value to cache
    Write,     // add the value in chache to the current block
    Reduce,    // subtract the cacle value from the current clock
    Add,       // add 1 to the cell
    Subtract,  // Subtract 1 from the cell
    MoveFront, // move fromt
    MoveBack,  // move back
    Display,   // print the u8 in stdout
    Number,    // Print the Number itself
    If, // run the instruction right after If if the read value is 0, else run the second instruction
    MarkOpen, // mark the place by number
    MarkClose, // goto back to respective mark if it reads 1
}

enum ReadError {
    Comment,
}

const SIZE_OF_MEMORY_IN_BYTES: usize = 1024 * 10; // 10KB

impl Instruction {
    fn from_file(file: &str) -> Vec<Instruction> {
        let file = match fs::read_to_string(file) {
            Ok(t) => t,
            Err(t) => panic!("Error while opening file\nERROR: {t}"),
        };

        Instruction::from_string(file)
    }

    fn from_string(string: String) -> Vec<Instruction> {
        let file_into_bytes = string.as_bytes();

        let mut return_vector: Vec<Instruction> = Vec::new();
        for byte in file_into_bytes {
            match Instruction::instruction_from_byte(*byte) {
                Ok(t) => return_vector.push(t),
                Err(_t) => {}
            }
        }

        return_vector
    }

    fn instruction_from_byte(s: u8) -> Result<Self, ReadError> {
        let s = s as char;

        match s {
            'R' => Ok(Instruction::Read),
            'W' => Ok(Instruction::Write),
            'T' => Ok(Instruction::Reduce),
            '+' => Ok(Instruction::Add),
            '-' => Ok(Instruction::Subtract),
            '>' => Ok(Instruction::MoveFront),
            '<' => Ok(Instruction::MoveBack),
            'D' => Ok(Instruction::Display),
            '?' => Ok(Instruction::If),
            'N' => Ok(Instruction::Number),
            '[' => Ok(Instruction::MarkOpen),
            ']' => Ok(Instruction::MarkClose),
            _ => Err(ReadError::Comment),
        }
    }
}

struct Machine {
    tape: [u8; SIZE_OF_MEMORY_IN_BYTES], // I am limited by the technology of my time
    head_location: usize,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            tape: [0; SIZE_OF_MEMORY_IN_BYTES], // make the memory
            head_location: 0,                   // make sure head starts at 0th index
        }
    }
}

struct InstructionMachine {
    instructions: Vec<Instruction>,
    head: usize,
}

impl InstructionMachine {
    fn new_from_file(file: &str) -> Self {
        InstructionMachine {
            instructions: Instruction::from_file(file),
            head: 0,
        }
    }
}

fn get_file_name() -> String {
    match env::var("TUPROG") {
        Ok(t) => t,
        Err(t) => {
            eprint!("[-] environment variable TUPROG not set, using default file \"program\" \nERROR: {t}");
            String::from("program")
        }
    }
}

fn main() {
    let filename = get_file_name();

    println!("the file is argument exists it is {filename}");

    let mut machine = Machine::new();

    let mut instruction_machine = InstructionMachine::new_from_file(&filename);

    let mut cache: u8 = 0;

    loop {
        execute_instruction(&mut machine, &mut instruction_machine, &mut cache);
    }
}

fn execute_instruction(
    machine_state: &mut Machine,
    instruction: &mut InstructionMachine,
    cache: &mut u8,
) {
    if instruction.head + 1 > instruction.instructions.len() {
        exit(0);
    }

    match &instruction.instructions[instruction.head] {
        Instruction::Read => {
            *cache = machine_state.tape[machine_state.head_location];
        }

        Instruction::Write => {
            let adder = *cache;
            machine_state.tape[machine_state.head_location] += adder;
        }

        Instruction::Reduce => {
            let subtracter = *cache;
            machine_state.tape[machine_state.head_location] -= subtracter;
        }

        Instruction::Add => {
            machine_state.tape[machine_state.head_location] += 1;
        }

        Instruction::Subtract => {
            machine_state.tape[machine_state.head_location] -= 1;
        }

        Instruction::MoveFront => {
            machine_state.head_location += 1;
        }
        Instruction::MoveBack => {
            machine_state.head_location -= 1;
        }
        Instruction::Display => {
            print!(
                "{}",
                machine_state.tape[machine_state.head_location] as char
            );
        }

        Instruction::Number => {
            print!("{}", machine_state.tape[machine_state.head_location]);
        }

        Instruction::If => {
            if machine_state.tape[machine_state.head_location] != 0 {
                instruction.head += 1;
                execute_instruction(machine_state, instruction, cache);
                instruction.head += 2;
            } else {
                instruction.head += 2;
                execute_instruction(machine_state, instruction, cache);
                instruction.head += 2;
            }
        }

        Instruction::MarkClose => {
            if machine_state.tape[machine_state.head_location] != 0 {
                let mut tempsortage: usize = 0;
                loop {
                    instruction.head -= 1;
                    let current_instruction = &instruction.instructions[instruction.head];
                    match current_instruction {
                        Instruction::MarkClose => tempsortage += 1,
                        Instruction::MarkOpen => {
                            if tempsortage == 0 {
                                break;
                            } else {
                                tempsortage -= 1;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        Instruction::MarkOpen => {}
    }

    instruction.head += 1;
}
