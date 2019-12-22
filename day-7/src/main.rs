mod app;
use app::*;
use intcode::*;
use std::fs::read_to_string;

fn main() {
    // load the program
    let opt: Opt = app().get_matches().into();
    let memory: Vec<i32> = read_to_string(opt.path)
        .expect("Failed to read the file")
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("Malformed input"))
        .collect();

    println!("Opt: {:?}", opt);
}
