mod app;
use crate::app::*;

fn main() {
    // get the options
    let app = app();
    let opt: Opt = app.get_matches().into();

    // max sure the options are valid
    if opt.min > opt.max {
        println!("Invalid value for <max>: The number was smaller the <min>");
        std::process::exit(1)
    }

    println!("{:?}", opt);
}
