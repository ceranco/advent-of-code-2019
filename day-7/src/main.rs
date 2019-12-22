mod app;
use app::*;

fn main() {
    let opt: Opt = app().get_matches().into();
    println!("Opt: {:?}", opt);
}
