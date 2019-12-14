#[derive(Clone, Copy)]
pub enum Opcode {
    Add = 1,
    Multiply = 2,
    Terminate = 99,
}

impl Opcode {
    pub fn from_i32(i: i32) -> Result<Self, ()> {
        match i {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            99 => Ok(Opcode::Terminate),
            _ => Err(()),
        }
    }
}

/// Represents an Intcode computer.
pub struct IntcodeComputer {
    memory: Vec<i32>,
}

impl IntcodeComputer {
    pub fn new(memory: Vec<i32>) -> Self {
        Self { memory }
    }

    /// Runs program and consumes the memory.
    ///
    /// This methods drops `self`, but it prevents needles copies
    /// of the memory.
    pub fn run_once(mut self) -> i32 {
        // run the program
        let mut pc = 0usize;
        loop {
            match Opcode::from_i32(self.memory[pc]) {
                Ok(opcode) => match opcode {
                    Opcode::Add => {
                        // get the addresses
                        let idx1 = self.memory[pc + 1] as usize;
                        let idx2 = self.memory[pc + 2] as usize;
                        let dst = self.memory[pc + 3] as usize;

                        // perform the operation
                        self.memory[dst] = self.memory[idx1] + self.memory[idx2];
                    }
                    Opcode::Multiply => {
                        // get the addresses
                        let idx1 = self.memory[pc + 1] as usize;
                        let idx2 = self.memory[pc + 2] as usize;
                        let dst = self.memory[pc + 3] as usize;

                        // perform the operation
                        self.memory[dst] = self.memory[idx1] * self.memory[idx2];
                    }
                    Opcode::Terminate => break,
                },
                Err(()) => panic!(),
            };
            pc += 4;
        }
        self.memory[0]
    }

    /// Runs program and without consuming the memory.
    ///
    /// Can be run multiple times, but requires copying the memory
    /// each time.
    pub fn run(&self) -> i32 {
        let pc = IntcodeComputer::new(self.memory.clone());
        pc.run_once()
    }

    /// Sets the noun and the verb of the program.
    pub fn set(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }
}
