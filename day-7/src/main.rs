mod app;
use app::*;
use intcode::*;
use std::{
    fs::read_to_string,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

#[derive(Debug, Clone)]
struct MultiReciever {
    receiver: Arc<Mutex<Receiver<i32>>>,
}

impl Input for MultiReciever {
    fn get(&mut self) -> Result<i32, StreamError> {
        self.receiver.lock().unwrap().get()
    }
}

impl From<Receiver<i32>> for MultiReciever {
    fn from(receiver: Receiver<i32>) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

struct Stream {
    receiver: MultiReciever,
    sender: Sender<i32>,
}

impl From<(Sender<i32>, Receiver<i32>)> for Stream {
    fn from((sender, receiver): (Sender<i32>, Receiver<i32>)) -> Self {
        Self {
            receiver: receiver.into(),
            sender,
        }
    }
}

fn generate_phase_settings(feedback: bool) -> [[i32; 5]; 120] {
    let offset = if feedback { 5 } else { 0 };
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
                                // phase_settings[counter] = [9,8,7,6,5];
                                phase_settings[counter] = [
                                    i1 + offset,
                                    i2 + offset,
                                    i3 + offset,
                                    i4 + offset,
                                    i5 + offset,
                                ];
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    phase_settings
}

fn regular_run(memory: Vec<i32>) {
    // prepare the io stream and amplifier
    let mut input: Stream = channel().into();
    let mut output: Stream = channel().into();
    let mut ampilfier = IntcodeComputer::new(memory, input.receiver, output.sender);

    // generate all the possible phase settings
    let phase_settings = generate_phase_settings(false);

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

fn feedback_run(memory: Vec<i32>) {
    // generate all the possible phase settings
    let phase_settings = generate_phase_settings(true);
    let mut highest_thrust_value = 0;
    let mut highest_thrust_idx = 0;

    for (idx, phase_setting) in phase_settings.iter().enumerate() {
        // create the streams that will communicate between the amplifiers
        let mut streams: Vec<Stream> = Vec::new();
        for _ in 0..5 {
            streams.push(channel().into());
        }
    
        // create the amplifiers and wire them up
        let mut amplifiers = Vec::new();
        for i in 0..5 {
            let input_idx = ((i as i32 - 1 + 5) % 5) as usize;
            let input = streams[input_idx].receiver.clone();
            let output = streams[i].sender.clone();

            // set the phase settings
            streams[input_idx].sender.push(phase_setting[i]);

            amplifiers.push(IntcodeComputer::new(memory.clone(), input, output));
        }
    
        // send the first signal (0) to amplifier A
        streams[4].sender.push(0);
    
        // run the amplifiers
        let mut join_handles = Vec::new();
        for mut amplifier in amplifiers.drain(..) {
            join_handles.push(thread::spawn(move || amplifier.run()));
        }
    
        // wait for the amplifiers to finish
        for handle in join_handles.drain(..) {
            handle.join().unwrap();
        }

        // get the thrust
        let thrust = streams[4].receiver.get().unwrap();
        if thrust > highest_thrust_value {
            highest_thrust_value = thrust;
            highest_thrust_idx = idx
        }
    }
    
    println!(
        "Highest thrust: {}, achieved with phase settings: {:?}",
        highest_thrust_value, phase_settings[highest_thrust_idx]
    );
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

    if opt.feedback {
        feedback_run(memory);
    } else {
        regular_run(memory);
    }
}
