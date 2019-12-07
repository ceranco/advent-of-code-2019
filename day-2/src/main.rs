use clap::{App, Arg};
use std::{
    ffi::{OsStr, OsString},
    fs::read_to_string,
    path::Path,
};

fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 2")
        .about("Emulates an Intcode computer")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("The path to the input file")
                .takes_value(true)
                .required(true)
                .validator_os(is_valid_path),
        )
}

#[derive(Clone, Copy)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Terminate = 99,
}

impl Opcode {
    fn from_i32(i: i32) -> Result<Self, ()> {
        match i {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            99 => Ok(Opcode::Terminate),
            _ => Err(()),
        }
    }
}

fn main() {
    let matches = app().get_matches();

    // open the input file, parse it and get the memory
    let mut memory: Vec<i32> = read_to_string(matches.value_of_os("input").unwrap())
        .expect("Failed to read the file")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Malformed input"))
        .collect();

    // restore the gravity assist program (your puzzle input) to the "1202 program alarm" state
    memory[1] = 12;
    memory[2] = 2;

    // run the program
    let mut pc= 0usize;
    loop {
        match Opcode::from_i32(memory[pc]) {
            Ok(opcode) => match opcode {
                Opcode::Add => {
                    // get the addresses
                    let idx1 = memory[pc + 1] as usize;
                    let idx2 = memory[pc + 2] as usize;
                    let dst = memory[pc + 3] as usize;

                    // perform the operation
                    memory[dst] = memory[idx1] + memory[idx2];
                }
                Opcode::Multiply => {
                    // get the addresses
                    let idx1 = memory[pc + 1] as usize;
                    let idx2 = memory[pc + 2] as usize;
                    let dst = memory[pc + 3] as usize;

                    // perform the operation
                    memory[dst] = memory[idx1] * memory[idx2];
                }
                Opcode::Terminate => break,
            },
            Err(()) => panic!(),
        };
        pc += 4;
    }

    println!("Memory[0]: {}", memory[0]);
}
