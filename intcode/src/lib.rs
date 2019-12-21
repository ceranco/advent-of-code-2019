use std::io::{self, BufRead, BufReader, Read, Stdin, Stdout, Write};

/// The parameter modes support by each `OpCode`.
///
/// Each parameter mode signals to the `IntcodeComputer` how to   
/// interpret the parameter value.
#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    /// In this mode, the value of the parameter will be interpreted
    /// as a memory location.
    ///
    /// # Example
    /// [`4`] - the value of the parameter will be interpreted as the value of memory location 4.
    Position,
    /// In this mode, the value of the parameter will be interpreted as a value.
    ///
    /// # Example
    /// [`4`] - the value of the parameter will be interpreted as simply 4.
    Immediate,
}

impl ParameterMode {
    fn from_i32(i: i32) -> Result<Self, ()> {
        match i {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(()),
        }
    }
}

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
#[derive(Clone, Copy, Debug)]
enum Opcode {
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
    Add(ParameterMode, ParameterMode),
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
    Multiply(ParameterMode, ParameterMode),
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
    Input,
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
    Output(ParameterMode),
    /// Sets the program counter to the second parameter (`loc`) if the first parameter is **non-zero** (`cond`).
    /// ```
    /// [5(JumpIfTrue), cond, loc]
    /// ```
    ///
    /// # Example
    /// ```
    /// [5, 1, 0, 99]
    /// ```
    /// This program will loop indefinitely.
    JumpIfTrue(ParameterMode, ParameterMode),
    /// Sets the program counter to the second parameter (`loc`) if the first parameter is **zero** (`cond`).
    /// ```
    /// [6(JumpIfFalse), cond, loc]
    /// ```
    ///
    /// # Example
    /// ```
    /// [6, 0, 0, 99]
    /// ```
    /// This program will loop indefinitely.
    JumpIfFalse(ParameterMode, ParameterMode),
    /// If the first parameter (`operand1`) is less than the second parameter (`operand2`),  
    /// it stores 1 in the position given by the third parameter (`dst`). Otherwise, it stores 0.
    /// ```
    /// [7(LessThan), operand1, operand2, dst]
    /// ```
    ///
    /// # Example
    /// ```
    /// [1107, 3, 4, 0, 99]
    /// ```
    /// This program will check if `3` is less than `4`, and if so store 1 in position `0` .   
    /// At the end of the program, the memory will look like:
    /// ```
    /// [1, 3, 4, 0, 99]
    /// ```
    LessThan(ParameterMode, ParameterMode),
    /// If the first parameter (`operand1`) is equal to the second parameter (`operand2`),  
    /// it stores 1 in the position given by the third parameter (`dst`). Otherwise, it stores 0.
    /// ```
    /// [8(Equals), operand1, operand2, dst]
    /// ```
    ///
    /// # Example
    /// ```
    /// [1108, 42, 42, 0, 1108, 42, 41, 1, 99]
    /// ```
    /// This program will make two comparisons:
    /// 1. `42` and `42`
    /// 2. `42` and `41`
    ///
    /// and stores the results in location `0` and `1` respectively:
    /// ```
    /// [1, 0, 42, 0, 1108, 42, 41, 1, 99]
    /// ```
    Equals(ParameterMode, ParameterMode),
    /// Terminates the program.
    ///
    /// # Example
    /// ```
    /// [99, 1, 0, 1, 0]
    /// ```
    /// This program does nothing, as it terminates after executing the   
    /// instruction in memory location `0`.
    Terminate,
}

impl Opcode {
    fn from_i32(i: i32) -> Result<Self, ()> {
        let opcode = i % 100; // get the right two digits
        let modes = (i / 100) // discard the right two digits
            .to_string()
            .chars() // split into digits
            .rev() // reverse as the parameter modes are specified from right to left
            .map(|c| ParameterMode::from_i32(c.to_digit(10).unwrap() as i32).unwrap()) // parse each digit into a parameter mode
            .collect::<Vec<ParameterMode>>();
        match opcode {
            1 => Ok(Opcode::Add(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            2 => Ok(Opcode::Multiply(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
            )),
            5 => Ok(Opcode::JumpIfTrue(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            6 => Ok(Opcode::JumpIfFalse(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            7 => Ok(Opcode::LessThan(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            8 => Ok(Opcode::Equals(
                *modes.get(0).unwrap_or(&ParameterMode::Position),
                *modes.get(1).unwrap_or(&ParameterMode::Position),
            )),
            99 => Ok(Opcode::Terminate),
            _ => Err(()),
        }
    }

    fn instruction_size(&self) -> usize {
        use Opcode::*;
        match self {
            Add(_, _) | Multiply(_, _) | LessThan(_, _) | Equals(_, _) => 4,
            Input | Output(_) => 2,
            Terminate => 1,
            JumpIfFalse(_, _) | JumpIfTrue(_, _) => 3,
        }
    }
}

/// Represents an Intcode computer.
///
/// Each instance will *own* its own memory (`Vec<i32>`).
///
/// # Example
/// ```
/// use intcode::*;
///
/// fn main() {
///     let memory = vec![1101, 40, 2, 0, 99];
///     let computer = IntcodeComputer::new(memory);
///
///     println!("The run finished with return value: {}", computer.run_once());
/// }
/// ```
///
/// This will create a new computer with a simple program that increments two numbers.  
/// At the end of the run (`run`/`run_once`) the value at location (0) of the memory will  
/// be returned.
///
/// *NOTE*: run_once consumes the memory, and as such can only be called once
pub struct IntcodeComputer<R: Read, W: Write> {
    memory: Vec<i32>,
    input: BufReader<R>,
    output: W,
}

impl IntcodeComputer<Stdin, Stdout> {
    /// Creates a new instance of `IntcodeComputer` that uses  
    /// `Stdin` and `Stdout` as the input and output streams.
    pub fn with_stdio(memory: Vec<i32>) -> Self {
        Self {
            memory,
            input: BufReader::new(io::stdin()),
            output: io::stdout(),
        }
    }
}

impl<R: Read, W: Write> IntcodeComputer<R, W> {
    pub fn new(memory: Vec<i32>, input: R, output: W) -> Self {
        Self {
            memory,
            input: BufReader::new(input),
            output,
        }
    }

    fn run_impl(&mut self) -> i32 {
        // helper function to get the correct value of a parameter
        fn get_value(memory: &[i32], idx: usize, mode: ParameterMode) -> i32 {
            match mode {
                ParameterMode::Position => memory[memory[idx] as usize],
                ParameterMode::Immediate => memory[idx],
            }
        }

        // run the program
        let mut pc = 0usize;
        loop {
            let opcode = Opcode::from_i32(self.memory[pc]).unwrap(); // get the opcode from the first two digits
            match opcode {
                Opcode::Add(src1_mode, src2_mode) => {
                    // get the parameters
                    let src1 = get_value(&self.memory, pc + 1, src1_mode);
                    let src2 = get_value(&self.memory, pc + 2, src2_mode);
                    let dst = self.memory[pc + 3] as usize; // always in position mode

                    // perform the operation
                    self.memory[dst] = src1 + src2;
                }
                Opcode::Multiply(src1_mode, src2_mode) => {
                    // get the parameters
                    let src1 = get_value(&self.memory, pc + 1, src1_mode);
                    let src2 = get_value(&self.memory, pc + 2, src2_mode);
                    let dst = self.memory[pc + 3] as usize; // always in position mode

                    // perform the operation
                    self.memory[dst] = src1 * src2;
                }
                Opcode::Input => {
                    // get the parameters
                    let mut input_str = String::new();
                    self.input
                        .read_line(&mut input_str)
                        .expect("Failed to read line");
                    let input: i32 = input_str.trim().parse().expect("Input was not i32"); // TODO: handle wrong input gracefully
                    let dst = self.memory[pc + 1] as usize; // always in position mode

                    // perform the operation
                    self.memory[dst] = input;
                }
                Opcode::Output(src_mode) => {
                    // get the parameter
                    let src = get_value(&self.memory, pc + 1, src_mode);

                    // perform the operation
                    self.output
                        .write_all(format!("{}\n", src).as_bytes())
                        .expect("Failed to output");
                }
                Opcode::JumpIfTrue(cond_mode, loc_mode) => {
                    // get the parameters
                    let cond = get_value(&self.memory, pc + 1, cond_mode);
                    let loc = get_value(&self.memory, pc + 2, loc_mode) as usize;

                    // perform the operation
                    if cond != 0 {
                        pc = loc.wrapping_sub(opcode.instruction_size());
                    }
                }
                Opcode::JumpIfFalse(cond_mode, loc_mode) => {
                    // get the parameters
                    let cond = get_value(&self.memory, pc + 1, cond_mode);
                    let loc = get_value(&self.memory, pc + 2, loc_mode) as usize;

                    // perform the operation
                    if cond == 0 {
                        pc = loc.wrapping_sub(opcode.instruction_size());
                    }
                }
                Opcode::LessThan(operand1_mode, operand2_mode) => {
                    // get the parameters
                    let operand1 = get_value(&self.memory, pc + 1, operand1_mode);
                    let operand2 = get_value(&self.memory, pc + 2, operand2_mode);
                    let dst = self.memory[pc + 3] as usize; // always in position mode

                    // perform the operation
                    self.memory[dst] = if operand1 < operand2 { 1 } else { 0 }
                }
                Opcode::Equals(operand1_mode, operand2_mode) => {
                    // get the parameters
                    let operand1 = get_value(&self.memory, pc + 1, operand1_mode);
                    let operand2 = get_value(&self.memory, pc + 2, operand2_mode);
                    let dst = self.memory[pc + 3] as usize; // always in position mode

                    // perform the operation
                    self.memory[dst] = if operand1 == operand2 { 1 } else { 0 }
                }
                Opcode::Terminate => break,
            };
            pc = pc.wrapping_add(opcode.instruction_size());
        }
        self.memory[0]
    }

    /// Runs program and consumes the memory.
    ///
    /// This methods drops `self`, but it prevents needless copies
    /// of the memory.
    pub fn run_once(mut self) -> i32 {
        self.run_impl()
    }

    /// Runs program and without consuming the memory.
    ///
    /// Can be run multiple times, but requires copying the memory
    /// each time.
    pub fn run(&mut self) -> i32 {
        // clone the memory to restore later
        let memory = self.memory.clone();
        let output = self.run_impl();

        // restore the memory
        self.memory = memory;

        output
    }

    /// Sets the noun and the verb of the program.
    pub fn set(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }
}
