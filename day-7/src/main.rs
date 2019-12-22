mod app;
mod stream;
use app::*;
use intcode::*;
use std::{
    fs::read_to_string,
    io::{Read, Write},
};
use stream::*;

fn main() {
    // load the program
    let opt: Opt = app().get_matches().into();
    let memory: Vec<i32> = read_to_string(opt.path)
        .expect("Failed to read the file")
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("Malformed input"))
        .collect();

    // prepare the io stream and amplifier
    let mut io_stream = Stream::new();
    let mut ampilfier = IntcodeComputer::new(memory, io_stream.clone(), io_stream.clone());

    // generate all the possible phase settings
    let mut phase_settings = [[0; 5]; 120];
    {
        let mut counter = 0;
        for i1 in 0..5 {
            for i2 in 0..5 {
                for i3 in 0..5 {
                    for i4 in 0..5 {
                        for i5 in 0..5 {
                            if i1 != i2
                                && i1 != i3
                                && i1 != i4
                                && i1 != i5
                                && i2 != i3
                                && i2 != i4
                                && i2 != i5
                                && i3 != i4
                                && i3 != i5
                                && i4 != i5
                            {
                                phase_settings[counter] = [i1, i2, i3, i4, i5];
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut highest_thrust_value = 0;
    let mut highest_thrust_idx = 0;
    for (idx, phase_setting) in phase_settings.iter().enumerate() {
        // setup the input
        io_stream
            .write_all(format!("{}\n0\n", phase_setting[0]).as_bytes())
            .unwrap();

        // run the amplifiers
        ampilfier.run(); // run the first one
        for i in 1..5 {
            let mut output = String::new();
            io_stream.read_to_string(&mut output).unwrap();

            let setting = phase_setting[i];
            // setup the input
            io_stream
                .write_all(format!("{}\n{}", setting, output).as_bytes())
                .unwrap();

            // run the amplifier
            ampilfier.run();
        }

        let mut value = String::new();
        io_stream.read_to_string(&mut value).unwrap();
        let value: i32 = value.trim().parse().unwrap();
        if value > highest_thrust_value {
            highest_thrust_value = value;
            highest_thrust_idx = idx
        }
    }

    println!(
        "Highest thrust: {}, achieved with phase settings: {:?}",
        highest_thrust_value, phase_settings[highest_thrust_idx]
    );
}
