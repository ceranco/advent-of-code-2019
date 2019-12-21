mod app;
use app::*;
use intcode::*;
use std::fs::read_to_string;

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
    let mut pc = IntcodeComputer::with_stdio(memory);

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
