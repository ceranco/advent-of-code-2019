#[derive(Clone, Copy)]
pub enum Opcode {
    /// Adds the numbers contained in two memory locations (`src1`, `src2`) and saves the sum in a third (`dst`):  
    /// ```
    /// [1(Add), src1, src2, dst]
    /// ````
    /// 
    /// # Example
    /// ```
    /// [1, 5, 6, 0, 99, 40, 2]
    /// ```
    /// This program adds the two numbers at memory locations `5` (40) and `6` (2),   
    /// saves the sum (42) in memory location `0` and then terminates.  
    /// At the end of the program, the memory will look like the following:
    /// ```
    /// [42, 5, 6, 0, 99, 40, 2]
    /// ```
    Add = 1,
    /// Multiplies the numbers contained in two memory locations (`src1`, `src2`) and saves the product in a third (`dst`):  
    /// ```
    /// [2(Multiply), src1, src2, dst]
    /// ````
    /// 
    /// # Example
    /// ```
    /// [2, 5, 6, 0, 99, 40, 2]
    /// ```
    /// This program multiplies the two numbers at memory locations `5` (40) and `6` (2),   
    /// saves the product (80) in memory location `0` and then terminates.  
    /// At the end of the program, the memory will look like the following:
    /// ```
    /// [80, 5, 6, 0, 99, 40, 2]
    /// ```
    Multiply = 2,
    /// Takes a single integer as input and saves it to memory location `dst`.
    /// ```
    /// [3(Input), dst]
    /// ```
    /// 
    /// # Example
    /// ```
    /// [3, 0, 99]
    /// ```
    /// This program will take a single input and save in location `0`.
    /// Given the input `42`, the memory will look like the following at the program's end:
    /// ```
    /// [42, 0, 99]
    /// ```
    Input = 3,
    /// Outputs a single integer value (`dst`).
    /// ```
    /// [4(Output), dst]
    /// ```
    /// 
    /// # Example
    /// ```
    /// [4, 0, 99]
    /// ```
    /// This program will output a single value in location `0` (4).
    Output = 4,
    /// Terminates the program.
    /// 
    /// # Example
    /// ```
    /// [99, 1, 0, 1, 0]
    /// ```
    /// This program does nothing, as it terminates after execution the   
    /// opcode in memory location `0`.
    Terminate = 99,
}

impl Opcode {
    pub fn from_i32(i: i32) -> Result<Self, ()> {
        match i {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
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
                    _ => unimplemented!()
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
