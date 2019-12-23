mod app;
use app::*;
use intcode::*;
use std::{
    fs::read_to_string,
    sync::mpsc::{channel, Receiver, Sender},
};

struct Stream {
    receiver: Receiver<i32>,
    sender: Sender<i32>,
}

impl From<(Sender<i32>, Receiver<i32>)> for Stream {
    fn from((sender, receiver): (Sender<i32>, Receiver<i32>)) -> Self {
        Self { receiver, sender }
    }
}

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
    let mut input: Stream = channel().into();
    let mut output: Stream = channel().into();
    let mut ampilfier = IntcodeComputer::new(memory, input.receiver, output.sender);

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
        input.sender.push(phase_setting[0]);
        input.sender.push(0);

        // run the amplifiers
        ampilfier.run(); // run the first one
        for setting in &phase_setting[1..5] {
            let output = output.receiver.get().unwrap();

            // setup the input
            input.sender.push(*setting);
            input.sender.push(output);

            // run the amplifier
            ampilfier.run();
        }

        let value = output.receiver.get().unwrap();
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
