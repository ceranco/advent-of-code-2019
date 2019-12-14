/// The opcodes supported by the `IntcodeComputer`.
///
/// # Parameter modes
/// The parameters of each code support two modes:
///
/// 1. *position* (`0`) - the parameter is interpreted as a **memory location**.
/// 2. *immediate* (`1`) - the parameter is intereted as a **value**.
///
/// If the parameter mode is **not** specified for a specific parameter,  
/// it defaults to *position* (`0`) mode.   
/// Parameters that the instruction writes to (`dst`) will **never**   
/// be in *immediate* (`1`) mode, **only** in *position* (`0`) mode.
///
/// Parameter modes are stored in the same value as the instruction's opcode.  
/// The opcode is the rightmost two digits of the first value in an instruction.  
/// Parameter modes are single digits, one per parameter, read right-to-left from the opcode.
///
/// ## Example
/// ```
/// [1002, 4, 3, 4, 33, 99]
/// ```
/// The first instruction, `[1002, 4, 3, 4]`, is a multiply instruction:   
/// the rightmost two digits of the first value, `02`, indicate multiplication.  
/// Then, going right to left, the parameter modes are `0`, `1` and `0` (not present, default to `0`)  
///
/// As such, the program will multiply the value at location `4`(33) with 3
/// and save the product (99) at location `4`:
/// ```
/// [1002, 4, 3, 4, 99, 99]
/// ```
#[derive(Clone, Copy)]
pub enum Opcode {
    /// Adds the numbers in parameters (`src1`, `src2`) and saves the sum in the location specified by (`dst`).
    /// ```
    /// [1(Add), src1(0|1), src2(0|1), dst(0)]
    /// ````
    ///
    /// # Example
    /// ```
    /// [1001, 5, 2, 0, 99, 40]
    /// ```
    /// This program adds the the number at location `5` with 2,   
    /// saves the sum (42) in memory location `0` and then terminates.  
    /// At the end of the program, the memory will look like the following:
    /// ```
    /// [42, 5, 2, 0, 99, 40]
    /// ```
    Add = 1,
    /// Multiplies the numbers in parameters (`src1`, `src2`) and saves the product in the location specified by (`dst`):  
    /// ```
    /// [2(Multiply), src1(0|1), src2(0|1), dst(0)]
    /// ````
    ///
    /// # Example
    /// ```
    /// [102, 40, 5, 0, 99, 2]
    /// ```
    /// This program multiplies the number 40 with the number at memory location `5` (2),  
    /// saves the product (80) in memory location `0` and then terminates.  
    /// At the end of the program, the memory will look like the following:
    /// ```
    /// [80, 40, 5, 0, 99, 2]
    /// ```
    Multiply = 2,
    /// Takes a single integer as input and saves it to memory location `dst`.
    /// ```
    /// [3(Input), dst(0)]
    /// ```
    ///
    /// # Example
    /// ```
    /// [3, 0, 99]
    /// ```
    /// This program will take a single input and save in location `0` .   
    /// Given the input `42`, the memory will look like the following at the program's end:
    /// ```
    /// [42, 0, 99]
    /// ```
    Input = 3,
    /// Outputs a single integer value in parameter (`src`).
    /// ```
    /// [4(Output), src(0|1)]
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
    /// This program does nothing, as it terminates after executing the   
    /// instruction in memory location `0`.
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
    /// This methods drops `self`, but it prevents needless copies
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
                    _ => unimplemented!(),
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
