use clap::{App, AppSettings, Arg, SubCommand};
use std::{
    ffi::{OsStr, OsString},
    fs::read_to_string,
    path::Path,
};
use intcode::*;

fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

fn is_valid_i32(input: String) -> Result<(), String> {
    match input.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Could not parse the given input to i32")),
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 2")
        .about("Emulates an Intcode computer")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs an Intcode program and prints the output")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                )
                .arg(
                    Arg::with_name("alarm")
                        .short("a")
                        .long("alarm")
                        .help("Runs the program in the '1202 program alarm' state")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("reverse")
                .about("Finds the noun and verb for a given output and Intcode program")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("The output that the program will search for")
                        .takes_value(true)
                        .required(true)
                        .validator(is_valid_i32),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn main() {
    let matches = app().get_matches();
    let subcommand_name = matches.subcommand().0;
    let subcommand_matches = matches.subcommand().1.unwrap();

    // open the input file, parse it and get the memory
    let memory: Vec<i32> = read_to_string(subcommand_matches.value_of_os("input").unwrap())
        .expect("Failed to read the file")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Malformed input"))
        .collect();
    let mut pc = IntcodeComputer::new(memory);

    match subcommand_name {
        "run" => {
            // optionally restore the gravity assist program to the "1202 program alarm" state
            if subcommand_matches.is_present("alarm") {
                pc.set(12, 2);
            }

            // run the program
            let output = pc.run_once();

            println!("Memory[0]: {}", output);
        }
        "reverse" => {
            // get the wanted output
            let wanted_output: i32 = subcommand_matches
                .value_of("output")
                .unwrap()
                .parse()
                .unwrap();

            let (noun, verb) = {
                let mut noun = -1;
                let mut verb = -1;

                'outer: for i in 0..100 {
                    for j in 0..100 {
                        pc.set(i, j);
                        let answer = pc.run();
                        if answer == wanted_output {
                            noun = i;
                            verb = j;
                            break 'outer;
                        }
                    }
                }
                (noun, verb)
            };
            if noun == -1 || verb == -1 {
                panic!("Could not find a noun an verb for the given output!");
            }
            println!(
                "Noun: {}, Verb: {}, Product: {}",
                noun,
                verb,
                noun * 100 + verb
            );
        }
        _ => panic!("Unknown subcommand"),
    }
}
